use std::collections::HashMap;

use bytecode::{Code, Func};
use parser::{Item, RawExpr};
use slotmap::SlotMap;

use crate::bytecode::Instr;

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
struct FnComping<'a, 's> {
    output: bytecode::Func<'a, 's>,
}

impl<'a, 's> FnComping<'a, 's> {
    fn add_instr(&mut self, instr: bytecode::Instr<'s>, expr: &'a parser::Expr<'s>) {
        debug_assert_eq!(self.output.code.len(), self.output.spans.len());
        self.output.code.push(instr);
        self.output.spans.push(expr);
    }

    fn push_expr(&mut self, expr: &'a parser::Expr<'s>) {
        match &**expr {
            RawExpr::Literal(l) => self.add_instr(Instr::LoadLit(**l), expr),
            RawExpr::Var(_) => todo!(),
            RawExpr::Call(_, _) => todo!(),
            RawExpr::BinOp(l, o, r) => {
                self.push_expr(l);
                self.push_expr(r);
                self.add_instr(Instr::BinOp(**o), expr);
            }
            RawExpr::UnaryOp(u, e) => {
                self.push_expr(e);
                self.add_instr(Instr::UnOp(**u), expr);
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
