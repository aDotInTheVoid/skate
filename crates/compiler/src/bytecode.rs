use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use parser::{Expr, Stmt};

mod debug2_impl;
mod debug_impl;

#[derive(Clone, Serialize, /*Deserialize,*/ Default)]
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
    pub n_args: usize,
    pub name: &'s str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Const {}

// https://docs.python.org/3/library/dis.html#opcode-BUILD_TUPLE

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, debug2::Debug)]
pub enum Instr<'s> {
    // Basic
    LoadLit(#[serde(borrow)] parser::Literal<'s>),
    Print,
    Pop,

    BinOp(parser::BinOp),
    UnOp(parser::UnaryOp),

    GetLocal(usize),
    SetLocal(usize),

    // Zero in these cases is invalid in the VM, but used
    // temporarily in the compiller
    JumpForward(usize),
    JumpBackward(usize),
    JumpForwardIfFalse(usize),

    MakeArray(usize),
    MakeMap(usize),

    ArrayAccess,
    FieldAccess(parser::Name<'s>),
    ArraySet,
    FieldSet(parser::Name<'s>),

    Call(FuncKey),
    Return,
}

slotmap::new_key_type! { pub struct FuncKey; }
slotmap::new_key_type! { pub struct ConstKey; }

impl debug2::Debug for FuncKey {
    fn fmt(&self, _: &mut debug2::Formatter<'_>) -> std::fmt::Result {
        unreachable!("Debug `Instruction` with `InstrDebug`")
    }
}

#[test]
fn sizes() {
    use std::mem::size_of;

    // TODO: Optimize
    //
    // We could store literals in a constant table to make them small
    assert_eq!(size_of::<Instr>(), 8 * 6);
}
