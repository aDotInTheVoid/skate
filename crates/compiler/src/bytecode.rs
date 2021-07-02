use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use parser::{Expr, Stmt};

#[derive(Debug, Clone, Serialize, /*Deserialize,*/ Default)]
/// 'a is the lifetime of the AST
/// 's is the lifetime of the source string
pub struct Code<'a, 's> {
    #[serde(borrow)]
    pub fns: SlotMap<FuncKey, Func<'a, 's>>,
}

#[derive(Debug, Clone, Serialize /*Deserialize,*/, EnumAsInner)]
pub enum AstLoc<'a, 's> {
    Expr(&'a Expr<'s>),
    Stmt(&'a Stmt<'s>),
    None,
}

#[derive(Debug, Clone, Serialize, /*Deserialize,*/ Default)]
pub struct Func<'a, 's> {
    // TODO: Whats this for?
    pub consts: SlotMap<ConstKey, Const>,
    #[serde(borrow)]
    pub code: Vec<Instr<'s>>,
    // TODO: Arena allocate the AST's so these can be keys and I can
    // Re-enable serde
    pub spans: Vec<AstLoc<'a, 's>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Const {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Instr<'s> {
    Print,
    // Return,
    Pop,
    LoadLit(#[serde(borrow)] parser::Literal<'s>),
    BinOp(parser::BinOp),
    UnOp(parser::UnaryOp),
    GetLocal(usize),
    SetLocal(usize),
    // Zero in these cases is invalid in the VM, but used
    // temporarily in the compiller
    JumpForward(usize),
    JumpBackward(usize),
    JumpForwardIfFalse(usize),
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
