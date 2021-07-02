use parser::{RawExpr, RawStmt};

use crate::bytecode::{AstLoc, Instr};

pub mod bytecode;

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

#[derive(Default, Debug)]
pub struct FnComping<'a, 's> {
    pub output: bytecode::Func<'a, 's>,
    locals: Vec<Local<'s>>,
    scope_depth: i32,
}

#[derive(Debug)]
struct Local<'s> {
    name: &'s str,
    depth: i32,
}

#[must_use]
#[derive(Debug)]
struct JumpHelper(usize);

#[must_use]
#[derive(Debug)]
struct BackJumpHelper(usize);

impl<'a, 's> FnComping<'a, 's> {
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

    fn add_loop(
        &mut self,
        ty: impl Fn(usize) -> Instr<'static>,
        stmt: &'a parser::Stmt<'s>,
    ) -> JumpHelper {
        self.add_instr_sloc(ty(0), stmt);
        JumpHelper(self.n_instrs())
    }

    fn add_jump_back_to_point(&self) -> BackJumpHelper {
        BackJumpHelper(self.output.code.len())
    }

    fn patch_fjump(&mut self, h: JumpHelper) {
        let current_ip = self.n_instrs();
        let diff = current_ip - h.0;

        if let Instr::JumpForward(i) | Instr::JumpForwardIfFalse(i) = &mut self.output.code[h.0 - 1]
        {
            *i = diff;
        } else {
            unreachable!("{:?}, {:?}", self.output.code, h);
        }
    }

    /// Add [`Instr::JumpForwardIfFalse`]
    fn add_jfif(&mut self, stmt: &'a parser::Stmt<'s>) -> JumpHelper {
        self.add_loop(Instr::JumpForwardIfFalse, stmt)
    }

    /// Add [`Instr::JumpForward`]
    fn add_jf(&mut self, stmt: &'a parser::Stmt<'s>) -> JumpHelper {
        self.add_loop(Instr::JumpForward, stmt)
    }

    fn add_jb(&mut self, pos: BackJumpHelper, stmt: &'a parser::Stmt<'s>) {
        let ip = self.output.code.len();
        let diff = ip - pos.0;
        self.add_instr_sloc(Instr::JumpBackward(diff), stmt)
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
            RawStmt::Return(_) => todo!(),
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
            RawStmt::For(_, _, _) => todo!(),
            RawStmt::While(cond, block) => {
                // http://craftinginterpreters.com/image/jumping-back-and-forth/while.png
                let loop_start = self.add_jump_back_to_point();
                self.push_expr(cond);
                let loop_exit = self.add_jfif(stmt);
                self.add_instr_sloc(Instr::Pop, stmt);
                self.add_block(block);
                self.add_jb(loop_start, stmt);
                self.patch_fjump(loop_exit);
            }
            RawStmt::Block(b) => self.add_block(b),
        }
    }

    pub fn push_expr(&mut self, expr: &'a parser::Expr<'s>) {
        match &**expr {
            RawExpr::Literal(l) => self.add_instr_eloc(Instr::LoadLit(**l), expr),
            RawExpr::Var(name) => {
                let name = unwrap_one(name);
                let id = self.resolve_local(name).expect("No Local Found");
                self.add_instr_eloc(Instr::GetLocal(id), expr);
            }
            RawExpr::Call(_, _) => todo!(),
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
            RawExpr::ArrayAccess(_, _) => todo!(),
            RawExpr::Array(_) => todo!(),
            RawExpr::Map(_) => todo!(),
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // TODO: Do this cleverer
        while self.scope_depth > 0 && self.locals.last().unwrap().depth > self.scope_depth {
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
    let mut comp = FnComping::default();
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
