use std::collections::HashMap;

use bytecode::{Code, FuncKey};
use parser::{Item, RawExpr, RawStmt};
use slotmap::SlotMap;

use crate::bytecode::{AstLoc, Instr};

pub mod bytecode;
mod utils;

// TODO: Make this failable

// pub fn compile(prog: parser::Program) -> Code {
//     let mut fn_map = SlotMap::with_key();
//     let mut fn_names = HashMap::new();

//     // For every function, obtain a key for it, and then map the name to a key.
//     for item in prog {
//         let item = match item {
//             Item::Function(_vis, f) => f,
//             Item::Import(_) => todo!(),
//             Item::Const(_, _, _) => todo!(),
//         };

//         let key = fn_map.insert(Default::default());
//         // TODO: Handle name collision like treewalk.
//         fn_names.insert(item.name.node, (key, item));
//     }

//     dbg!(fn_names, &fn_map);

//     Code { fns: fn_map }
// }

#[derive(Debug, Clone, Copy)]
struct FnInfo {
    n_args: usize,
    key: FuncKey,
}

pub fn compile<'a, 's>(prog: &'a parser::ProgramSlice<'s>) -> (Code<'a, 's>, bytecode::FuncKey) {
    let mut fns = SlotMap::with_key();
    let mut fn_names = HashMap::new();
    for (_, f) in prog.iter().filter_map(Item::as_function) {
        let key = fns.insert(Default::default());
        fn_names.insert(
            f.name.node,
            FnInfo {
                key,
                n_args: f.args.len(),
            },
        );
    }

    for (_, f) in prog.iter().filter_map(Item::as_function) {
        let id = fn_names[f.name.node].key;
        let func = build_function(f, &fn_names);
        fns[id] = func;
    }

    let main_key = fn_names["main"].key;

    (Code { fns }, main_key)
}

type FuncLut<'s> = HashMap<&'s str, FnInfo>;

#[derive(Debug)]
struct FnComping<'a, 's, 'l> {
    output: bytecode::Func<'a, 's>,
    locals: Vec<Local<'s>>,
    scope_depth: i32,
    func_map: &'l FuncLut<'s>,
}

#[derive(Debug)]
struct Local<'s> {
    name: &'s str,
    depth: i32,
}

fn build_function<'a, 's>(f: &'a parser::Function<'s>, l: &FuncLut<'s>) -> bytecode::Func<'a, 's> {
    let mut comp = FnComping::new(l);
    // This feals dirty, and like abuse of default
    comp.output.n_args = f.args.len();
    comp.output.name = f.name.node;

    for i in &f.args.node {
        comp.locals.push(Local {
            // TODO: Check names are unique
            depth: 1,
            name: i.name.node,
        })
    }

    match &f.body {
        parser::FnBody::Expr(e) => {
            comp.push_expr(e);
            comp.add_instr_nloc(Instr::Return);
        }
        parser::FnBody::Block(b) => {
            comp.add_block(b);
            comp.add_instr_nloc(Instr::LoadLit(parser::Literal::Null));
            comp.add_instr_nloc(Instr::Return);
        }
    }
    comp.output
}

impl<'a, 's, 'l> FnComping<'a, 's, 'l> {
    fn new(func_map: &'l FuncLut<'s>) -> Self {
        Self {
            output: Default::default(),
            locals: Default::default(),
            scope_depth: 0,
            func_map,
        }
    }

    fn add_block(&mut self, block: &'a parser::Block<'s>) {
        self.begin_scope();
        for i in &*block.node {
            self.push_stmt(i);
        }
        self.end_scope();
    }

    fn add_instr_eloc(&mut self, instr: Instr<'s>, expr: &'a parser::Expr<'s>) {
        debug_assert_eq!(self.output.code.len(), self.output.spans.len());
        self.output.code.push(instr);
        self.output.spans.push(AstLoc::Expr(expr));
    }

    fn add_instr_sloc(&mut self, instr: Instr<'s>, stmt: &'a parser::Stmt<'s>) {
        debug_assert_eq!(self.output.code.len(), self.output.spans.len());
        self.output.code.push(instr);
        self.output.spans.push(AstLoc::Stmt(stmt));
    }

    // TODO: Maybe remove
    fn add_instr_nloc(&mut self, instr: Instr<'s>) {
        debug_assert_eq!(self.output.code.len(), self.output.spans.len());
        self.output.code.push(instr);
        self.output.spans.push(AstLoc::None);
    }

    fn n_instrs(&self) -> usize {
        self.output.code.len()
    }

    pub fn push_stmt(&mut self, stmt: &'a parser::Stmt<'s>) {
        match &**stmt {
            RawStmt::Let(name, expr) => {
                for el in self.locals.iter().rev() {
                    if el.depth != -1 && el.depth < self.scope_depth {
                        break;
                    }
                    if el.name == name.node {
                        // TODO: Handle error
                        panic!("Duplicated name");
                    }
                }
                self.locals.push(Local {
                    // This cannot be used in the following expression
                    depth: -1,
                    name: &*name,
                });
                self.push_expr(expr);
                self.locals.last_mut().unwrap().depth = self.scope_depth;
            }

            RawStmt::Assign(name, expr) => {
                // TODO: Handle lvalue
                let name = unwrap_one(name.as_var().unwrap());

                let id = self.resolve_local(name).expect("No Local Found");
                self.push_expr(expr);
                self.add_instr_sloc(Instr::SetLocal(id), stmt);
                self.add_instr_sloc(Instr::Pop, stmt);
            }
            RawStmt::Expr(e) => {
                self.push_expr(e);
                self.add_instr_sloc(Instr::Pop, stmt);
            }
            RawStmt::Print(e) => {
                self.push_expr(e);
                self.add_instr_sloc(Instr::Print, stmt);
            }
            RawStmt::If(cond, tcase, fcase) => {
                // TODO: Optimize for case with no `fcase`
                // See: http://craftinginterpreters.com/image/jumping-back-and-forth/full-if-else.png
                self.push_expr(cond);
                let j1 = self.add_jfif(stmt);
                self.add_instr_sloc(Instr::Pop, stmt);
                self.add_block(tcase);
                let j2 = self.add_jf(stmt);
                self.patch_fjump(j1);
                self.add_instr_sloc(Instr::Pop, stmt);
                if let Some(fcase) = fcase {
                    self.add_block(fcase);
                }
                self.patch_fjump(j2);
            }
            RawStmt::While(cond, block) => {
                // http://craftinginterpreters.com/image/jumping-back-and-forth/while.png
                let loop_start = self.add_jump_back_to_point();
                self.push_expr(cond);
                let loop_exit = self.add_jfif(stmt);
                self.add_instr_sloc(Instr::Pop, stmt);
                self.add_block(block);
                self.add_jb(loop_start, stmt);
                self.patch_fjump(loop_exit);
                self.add_instr_sloc(Instr::Pop, stmt);
            }
            RawStmt::Block(b) => self.add_block(b),
            RawStmt::Return(expr) => {
                self.push_expr(expr);
                self.add_instr_sloc(Instr::Return, stmt);
            }
            RawStmt::For(_, _, _) => todo!(),
        }
    }

    pub fn push_expr(&mut self, expr: &'a parser::Expr<'s>) {
        match &**expr {
            RawExpr::Literal(l) => self.add_instr_eloc(Instr::LoadLit(**l), expr),
            RawExpr::Var(name) => {
                let name = unwrap_one(name);
                let id = self
                    .resolve_local(name)
                    .unwrap_or_else(|| panic!("Local {} not found", name.node));
                self.add_instr_eloc(Instr::GetLocal(id), expr);
            }
            RawExpr::BinOp(l, o, r) => {
                self.push_expr(l);
                self.push_expr(r);
                self.add_instr_eloc(Instr::BinOp(**o), expr);
            }
            RawExpr::UnaryOp(u, e) => {
                self.push_expr(e);
                self.add_instr_eloc(Instr::UnOp(**u), expr);
            }
            RawExpr::FieldAccess(_, _) => todo!(),
            RawExpr::ArrayAccess(array, index) => {
                self.push_expr(array);
                self.push_expr(index);
                self.add_instr_eloc(Instr::ArrayAccess, expr)
            }
            RawExpr::Array(items) => {
                for i in items {
                    self.push_expr(i);
                }
                self.add_instr_eloc(Instr::MakeArray(items.len()), expr);
            }
            RawExpr::Map(_) => todo!(),
            RawExpr::Call(path, args) => {
                let func = unwrap_one(path.as_var().unwrap());
                let func_info = self.func_map[func.node];
                assert_eq!(func_info.n_args, args.len());
                for i in args {
                    self.push_expr(i);
                }
                self.add_instr_eloc(Instr::Call(func_info.key), expr)
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // TODO: Do this cleverer
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.add_instr_nloc(Instr::Pop);
            self.locals.pop();
        }
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (idx, loc) in self.locals.iter().enumerate().rev() {
            if loc.depth != -1 && loc.name == name {
                return Some(idx);
            }
        }
        None
    }
}

#[test]
fn basic_expr() {
    let code = "2 * -3 / 5";
    let expr = parser::ExprParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();

    let m = HashMap::new();
    let mut comp = FnComping::new(&m);
    comp.push_expr(&expr);
    let func = comp.output;
    assert_eq!(
        func.code,
        vec![
            Instr::LoadLit(parser::Literal::Integer(2)),
            Instr::LoadLit(parser::Literal::Integer(3)),
            Instr::UnOp(parser::UnaryOp::Minus),
            Instr::BinOp(parser::BinOp::Times),
            Instr::LoadLit(parser::Literal::Integer(5)),
            Instr::BinOp(parser::BinOp::Devide)
        ]
    );
}

fn unwrap_one<T>(x: &[T]) -> &T {
    assert_eq!(x.len(), 1);
    &x[0]
}
