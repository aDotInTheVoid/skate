use parser::Expr;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

#[derive(Debug, Clone, Serialize, /*Deserialize,*/ Default)]
/// 'a is the lifetime of the AST
/// 's is the lifetime of the source string
pub struct Code<'a, 's> {
    #[serde(borrow)]
    pub fns: SlotMap<FuncKey, Func<'a, 's>>,
}

#[derive(Debug, Clone, Serialize, /*Deserialize,*/ Default)]
pub struct Func<'a, 's> {
    // TODO: Whats this for?
    pub consts: SlotMap<ConstKey, Const>,
    #[serde(borrow)]
    pub code: Vec<Instr<'s>>,
    // TODO: Arena allocate Expr's so these can be keys and I can
    // Re-enable serde
    pub spans: Vec<&'a Expr<'s>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Const {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Instr<'s> {
    Print,
    Return,
    LoadLit(#[serde(borrow)] parser::Literal<'s>),
    BinOp(parser::BinOp),
    UnOp(parser::UnaryOp),
}

slotmap::new_key_type! { pub struct FuncKey; }
slotmap::new_key_type! { pub struct ConstKey; }

#[test]
fn sizes() {
    use std::mem::size_of;

    // TODO: Optimize
    //
    // We could store literals in a constant table to make them small
    assert_eq!(size_of::<Instr>(), 8 * 4);
}
