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

#[derive(Default)]
pub struct FnComping<'a, 's> {
    pub output: bytecode::Func<'a, 's>,
}

impl<'a, 's> FnComping<'a, 's> {
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

    pub fn push_stmt(&mut self, stmt: &'a parser::Stmt<'s>) {
        match &**stmt {
            RawStmt::Let(_, _) => todo!(),
            RawStmt::Assign(_, _) => todo!(),
            RawStmt::Expr(e) => {
                self.push_expr(e);
                self.add_instr_sloc(Instr::Pop, stmt);
            }
            RawStmt::Print(e) => {
                self.push_expr(e);
                self.add_instr_sloc(Instr::Print, stmt);
            }
            RawStmt::Return(_) => todo!(),
            RawStmt::If(_, _, _) => todo!(),
            RawStmt::For(_, _, _) => todo!(),
            RawStmt::While(_, _) => todo!(),
            RawStmt::Block(_) => todo!(),
        }
    }

    pub fn push_expr(&mut self, expr: &'a parser::Expr<'s>) {
        match &**expr {
            RawExpr::Literal(l) => self.add_instr_eloc(Instr::LoadLit(**l), expr),
            RawExpr::Var(_) => todo!(),
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
