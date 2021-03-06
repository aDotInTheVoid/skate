use std::collections::HashMap;

use codespan_reporting::diagnostic::Diagnostic;
use diagnostics::span::Span;
use itertools::Itertools;
use slotmap::SlotMap;

use diagnostics::CompError;
use parser::{Item, RawExpr, RawStmt};

use crate::bytecode::{AstLoc, Code, FuncKey, Instr};

pub mod bytecode;
mod utils;

#[derive(Debug, Clone, Copy)]
struct FnInfo {
    n_args: usize,
    key: FuncKey,
    span: Span,
}

// TODO: Report multiple errors
pub fn compile<'a, 's>(
    prog: &'a parser::ProgramSlice<'s>,
) -> Result<(Code<'a, 's>, bytecode::FuncKey), CompError> {
    let mut fns = SlotMap::with_key();
    let mut fn_names = HashMap::new();
    for (_, f) in prog.iter().filter_map(Item::as_function) {
        let key = fns.insert(Default::default());
        if let Some(old_info) = fn_names.insert(
            f.name.node,
            FnInfo {
                key,
                n_args: f.args.len(),
                span: f.span,
            },
        ) {
            return Err(CompError(
                Diagnostic::error()
                    .with_message(format!("Function `{}` defined twice", f.name.node))
                    .with_labels(vec![
                        old_info
                            .span
                            .secondary_label()
                            .with_message("Defined once here"),
                        f.span.secondary_label().with_message("Defined again here"),
                    ]),
            ));
        }
    }

    for (_, f) in prog.iter().filter_map(Item::as_function) {
        let id = fn_names[f.name.node].key;
        let func = build_function(f, &fn_names)?;
        fns[id] = func;
    }

    let main_key = fn_names["main"].key;

    Ok((Code { fns }, main_key))
}

type FuncLut<'s> = HashMap<&'s str, FnInfo>;

#[derive(Debug)]
struct FnComping<'a, 's, 'l> {
    output: bytecode::Func<'a, 's>,
    locals: Vec<Local<'s>>,
    scope_depth: u64,
    func_map: &'l FuncLut<'s>,
}

#[derive(Debug)]
struct Local<'s> {
    name: &'s str,
    depth: Option<u64>,
}

fn build_function<'a, 's>(
    f: &'a parser::Function<'s>,
    l: &FuncLut<'s>,
) -> Result<bytecode::Func<'a, 's>, CompError> {
    let mut comp = FnComping::new(l, f.name.node, f.args.len());

    for i in &f.args.node {
        comp.locals.push(Local {
            // TODO: Check names are unique
            depth: Some(1),
            name: i.name.node,
        })
    }

    match &f.body {
        parser::FnBody::Expr(e) => {
            comp.push_expr(e)?;
            comp.add_instr_nloc(Instr::Return);
        }
        parser::FnBody::Block(b) => {
            comp.add_block(b)?;
            comp.add_instr_nloc(Instr::LoadLit(parser::Literal::Null));
            comp.add_instr_nloc(Instr::Return);
        }
    }
    Ok(comp.output)
}

impl<'a, 's, 'l> FnComping<'a, 's, 'l> {
    fn new(func_map: &'l FuncLut<'s>, name: &'s str, n_args: usize) -> Self {
        Self {
            output: bytecode::Func {
                code: Vec::new(),
                spans: Vec::new(),
                name,
                n_args,
            },
            locals: Default::default(),
            scope_depth: 0,
            func_map,
        }
    }

    fn add_block(&mut self, block: &'a parser::Block<'s>) -> Result<(), CompError> {
        self.begin_scope();
        for i in &*block.node {
            self.push_stmt(i)?;
        }
        self.end_scope();
        Ok(())
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

    pub fn push_stmt(&mut self, stmt: &'a parser::Stmt<'s>) -> Result<(), CompError> {
        match &**stmt {
            RawStmt::Let(name, expr) => {
                // TODO: Decide how I feal about shadowing, and why theirs
                // a `let` keyword in the first place.

                self.locals.push(Local {
                    // This cannot be used in the following expression
                    depth: None,
                    name: &*name,
                });
                self.push_expr(expr)?;
                self.locals.last_mut().unwrap().depth = Some(self.scope_depth);
            }

            RawStmt::Assign(name, expr) => match &name.node {
                RawExpr::Var(name) => {
                    let name = unwrap_one(name);
                    self.push_expr(expr)?;
                    let id = self.resolve_local(name)?;
                    self.add_instr_sloc(Instr::SetLocal(id), stmt);
                    self.add_instr_sloc(Instr::Pop, stmt);
                }
                RawExpr::FieldAccess(map, key) => {
                    self.push_expr(map)?;
                    self.push_expr(expr)?;
                    self.add_instr_sloc(Instr::FieldSet(*key), stmt);
                }
                RawExpr::ArrayAccess(array, index) => {
                    self.push_expr(array)?;
                    self.push_expr(index)?;
                    self.push_expr(expr)?;
                    self.add_instr_sloc(Instr::ArraySet, stmt)
                }
                _ => {
                    return Err(CompError(
                        Diagnostic::error()
                            .with_message("Excpected an lvalue")
                            .with_labels(vec![name.span.primary_label()]),
                    ))
                }
            },
            RawStmt::Expr(e) => {
                self.push_expr(e)?;
                self.add_instr_sloc(Instr::Pop, stmt);
            }
            RawStmt::Print(e) => {
                self.push_expr(e)?;
                self.add_instr_sloc(Instr::Print, stmt);
            }
            RawStmt::If(cond, tcase, fcase) => {
                // TODO: Optimize for case with no `fcase`
                // See: http://craftinginterpreters.com/image/jumping-back-and-forth/full-if-else.png
                self.push_expr(cond)?;
                let j1 = self.add_jfif(stmt);
                self.add_instr_sloc(Instr::Pop, stmt);
                self.add_block(tcase)?;
                let j2 = self.add_jf(stmt);
                self.patch_fjump(j1);
                self.add_instr_sloc(Instr::Pop, stmt);
                if let Some(fcase) = fcase {
                    self.add_block(fcase)?;
                }
                self.patch_fjump(j2);
            }
            RawStmt::While(cond, block) => {
                // http://craftinginterpreters.com/image/jumping-back-and-forth/while.png
                let loop_start = self.add_jump_back_to_point();
                self.push_expr(cond)?;
                let loop_exit = self.add_jfif(stmt);
                self.add_instr_sloc(Instr::Pop, stmt);
                self.add_block(block)?;
                self.add_jb(loop_start, stmt);
                self.patch_fjump(loop_exit);
                self.add_instr_sloc(Instr::Pop, stmt);
            }
            RawStmt::Block(b) => self.add_block(b)?,
            RawStmt::Return(expr) => {
                self.push_expr(expr)?;
                self.add_instr_sloc(Instr::Return, stmt);
            }
            RawStmt::For(_, _, _) => todo!(),
        }
        Ok(())
    }

    pub fn push_expr(&mut self, expr: &'a parser::Expr<'s>) -> Result<(), CompError> {
        match &**expr {
            RawExpr::Literal(l) => self.add_instr_eloc(Instr::LoadLit(**l), expr),
            RawExpr::Var(name) => {
                let name = unwrap_one(name);
                let id = self.resolve_local(name)?;
                self.add_instr_eloc(Instr::GetLocal(id), expr);
            }
            RawExpr::BinOp(l, o, r) => {
                self.push_expr(l)?;
                self.push_expr(r)?;
                self.add_instr_eloc(Instr::BinOp(**o), expr);
            }
            RawExpr::UnaryOp(u, e) => {
                self.push_expr(e)?;
                self.add_instr_eloc(Instr::UnOp(**u), expr);
            }
            RawExpr::FieldAccess(map, name) => {
                self.push_expr(map)?;
                self.add_instr_eloc(Instr::FieldAccess(*name), expr);
            }
            RawExpr::ArrayAccess(array, index) => {
                self.push_expr(array)?;
                self.push_expr(index)?;
                self.add_instr_eloc(Instr::ArrayAccess, expr)
            }
            RawExpr::Array(items) => {
                for i in items {
                    self.push_expr(i)?;
                }
                self.add_instr_eloc(Instr::MakeArray(items.len()), expr);
            }
            RawExpr::Map(m) => {
                let mut names = HashMap::new();

                for (k, v) in m {
                    if let Some(old_span) = names.insert(k.node, k.span) {
                        return Err(CompError(
                            Diagnostic::error()
                                .with_message(format!("Duplicate key `{}` in map literal", k.node))
                                .with_labels(vec![
                                    old_span.primary_label().with_message("First defined here"),
                                    k.span.primary_label().with_message("Defined here again"),
                                ]),
                        ));
                    }

                    // TODO: Give the span of the strings.
                    self.add_instr_eloc(Instr::LoadLit(parser::Literal::String(k.node)), expr);
                    self.push_expr(v)?;
                }
                self.add_instr_eloc(Instr::MakeMap(m.len()), expr)
            }
            RawExpr::Call(path, args) => {
                let func = unwrap_one(path.as_var().ok_or_else(|| {
                    CompError(
                        Diagnostic::error()
                            .with_message("Expected a name")
                            .with_labels(vec![path.primary_label()]),
                    )
                })?);
                let func_info = self.func_map.get(func.node).ok_or_else(|| {
                    CompError(
                        Diagnostic::error()
                            .with_message(format!("No function named `{}`", func.node))
                            .with_labels(vec![func.span.primary_label()]),
                    )
                })?;

                for i in args {
                    self.push_expr(i)?;
                }
                if func_info.n_args != args.len() {
                    return Err(CompError(
                        Diagnostic::error()
                            .with_message(format!(
                                "Expected {} args, found {}",
                                func_info.n_args,
                                args.len()
                            ))
                            .with_labels(vec![
                                expr.primary_label().with_message(format!(
                                    "Calling here with {} arguments",
                                    args.len()
                                )),
                                func_info.span.secondary_label().with_message(format!(
                                    "Defined here with {} arguments",
                                    func_info.n_args
                                )),
                            ]),
                    ));
                }

                self.add_instr_eloc(Instr::Call(func_info.key), expr)
            }
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // TODO: Do this cleverer
        while !self.locals.is_empty()
            && self.locals.last().unwrap().depth.unwrap() > self.scope_depth
        {
            self.add_instr_nloc(Instr::Pop);
            self.locals.pop();
        }
    }

    fn resolve_local(&self, name: &parser::Name<'_>) -> Result<usize, CompError> {
        for (idx, loc) in self.locals.iter().enumerate().rev() {
            if loc.depth.is_some() && loc.name == name.node {
                return Ok(idx);
            }
        }
        Err(CompError(
            Diagnostic::error()
                .with_message(format!("Couldn't find variable `{}` in scope", name.node))
                .with_labels(vec![name.span.primary_label()])
                .with_notes(vec![format!(
                    "Variables in scope: {:?}",
                    // This is kind of complicated logic to group
                    // vars into scope, and print them sorted,
                    // not sure if its a good idea.
                    self.locals
                        .iter()
                        .rev()
                        .filter(|local| local.depth.is_some())
                        .group_by(|local| local.depth)
                        .into_iter()
                        .flat_map(|(_, locals)| {
                            let mut names = locals.map(|l| l.name).collect_vec();
                            names.sort_unstable();
                            names
                        })
                        .collect_vec()
                )]),
        ))
    }
}

#[test]
fn basic_expr() {
    let code = "2 * -3 / 5";
    let expr = parser::ExprParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();

    let m = HashMap::new();
    let mut comp = FnComping::new(&m, "DEMO", 0);
    comp.push_expr(&expr).unwrap();
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
