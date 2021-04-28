use std::fmt::Write;
use std::ops::Range;
use std::usize;

use codespan_reporting::diagnostic::Label;
use serde::{Deserialize, Serialize};

use crate::diagnostics;

// TODO: Clean up "outer" vs inner spans
// Eg an Expr has its own assocd span, but a BinOp doesnt

pub type Block<'a> = Spanned<(Vec<Stmt<'a>>, BlockType)>;
pub type Program<'a> = Vec<Item<'a>>;
pub type Name<'a> = Spanned<&'a str>;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum BlockType {
    ReturnExpr,
    Discard,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file_id: diagnostics::FileId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl Span {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn primary_label(&self) -> Label<usize> {
        Label::primary(self.file_id.0, self.range())
    }

    pub fn secondary_label(&self) -> Label<usize> {
        Label::secondary(self.file_id.0, self.range())
    }
}

impl<T> Spanned<T> {
    pub fn primary_label(&self) -> Label<usize> {
        self.span.primary_label()
    }

    pub fn secondary_label(&self) -> Label<usize> {
        self.span.secondary_label()
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
    pub ty: Option<Spanned<Type>>,
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

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinOp::Equals => "==",
            BinOp::NotEquals => "!=",
            BinOp::LessThan => "<",
            BinOp::GreaterThan => ">",
            BinOp::LessThanEquals => "<=",
            BinOp::GreaterThanEquals => ">=",
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Times => "*",
            BinOp::Devide => "/",
            BinOp::LogicalOr => "||",
            BinOp::LogicalAnd => "&&",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum UnaryOp {
    Not,
    Minus,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            UnaryOp::Not => '!',
            UnaryOp::Minus => '-',
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
}
