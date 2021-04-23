use std::ops::Range;
use std::usize;

use codespan_reporting::diagnostic::Label;
use serde::{Deserialize, Serialize};

use crate::diagnostics;

// TODO: Clean up "outer" vs inner spans
// Eg an Expr has its own assocd span, but a BinOp doesnt

pub type Block<'a> = Spanned<Vec<Stmt<'a>>>;
pub type Program<'a> = Vec<Item<'a>>;
pub type Name<'a> = Spanned<&'a str>;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file_id: diagnostics::FileId,
}

impl Span {
    pub fn to_range(&self) -> Range<usize> {
        self.start..self.end
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn primary_label(&self) -> Label<usize> {
        Label::primary(self.span.file_id.0, self.span.to_range())
    }

    pub fn secondary_label(&self) -> Label<usize> {
        Label::secondary(self.span.file_id.0, self.span.to_range())
    }
}

// TODO: Decide if this is a good idea
impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
impl<T> std::ops::DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item<'a> {
    Function(#[serde(borrow)] Spanned<Function<'a>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function<'a> {
    #[serde(borrow)]
    pub name: Name<'a>,
    #[serde(borrow)]
    pub args: Spanned<Vec<Arg<'a>>>,
    pub ret: Option<Spanned<Type>>,
    #[serde(borrow)]
    pub body: Block<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg<'a> {
    #[serde(borrow)]
    pub name: Name<'a>,
    pub ty: Spanned<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Bool,
    Int,
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt<'a> {
    Let(#[serde(borrow)] Name<'a>, Expr<'a>),
    Expr(Expr<'a>),
    Print(Expr<'a>),
    Return(Expr<'a>),
}

pub type Expr<'a> = Spanned<RawExpr<'a>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawExpr<'a> {
    Literal(Spanned<Literal<'a>>),
    Var(Name<'a>),
    Block(Block<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Spanned<BinOp>, Box<Expr<'a>>),
    UnaryOp(Spanned<UnaryOp>, Box<Expr<'a>>),
    FieldAccess(Box<Expr<'a>>, #[serde(borrow)] Name<'a>),
    ArrayAccess(Box<Expr<'a>>, Box<Expr<'a>>),
    // Test, truecase, falsecase
    If(Box<Expr<'a>>, Block<'a>, Option<Block<'a>>),
    For(Name<'a>, Box<Expr<'a>>, Block<'a>),
    While(Box<Expr<'a>>, Block<'a>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BinOp {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Plus,
    Minus,
    Times,
    Devide,
    LogicalOr,
    LogicalAnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Bool(bool),
}
