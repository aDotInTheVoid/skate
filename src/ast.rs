use std::fmt::Write;
use std::ops::Range;
use std::usize;

use codespan_reporting::diagnostic::Label;
use serde::{Deserialize, Serialize};

use crate::{diagnostics, env, value};

// TODO: Clean up "outer" vs inner spans
// Eg an Expr has its own assocd span, but a Function doesnt

// https://golang.org/ref/spec
// https://github.com/katef/kgt/blob/main/examples/c99-grammar.iso-ebnf
// https://katef.github.io/kgt/doc/gallery/c99-ebnf.html

// TODO: Check out https://crates.io/crates/rowan and https://crates.io/crates/ungrammar

// 'a is the lifetime of the source string
pub type Block<'a> = Spanned<Vec<Stmt<'a>>>;
pub type Program<'a> = Vec<Item<'a>>;
pub type Name<'a> = Spanned<&'a str>;

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

    pub(crate) fn evaled_to_primary(&self, v: value::Value, e: &env::Env) -> Label<usize> {
        self.primary_label()
            .with_message(format!("Evaluated to `{:?}`", e.dbg_val(&v)))
    }

    pub(crate) fn evaled_to(&self, v: value::Value, e: &env::Env) -> Label<usize> {
        self.secondary_label()
            .with_message(format!("Evaluated to `{:?}`", e.dbg_val(&v)))
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
    pub body: FnBody<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FnBody<'a> {
    Expr(#[serde(borrow)] Expr<'a>),
    Block(#[serde(borrow)] Block<'a>),
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

pub type Stmt<'a> = Spanned<RawStmt<'a>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawStmt<'a> {
    Let(#[serde(borrow)] Name<'a>, Expr<'a>),
    // TODO: Dont use Expr for Lvalues
    Assign(Expr<'a>, Expr<'a>),
    Expr(Expr<'a>),
    Print(Expr<'a>),
    Return(Expr<'a>),
    // Test, truecase, falsecase
    If(Box<Expr<'a>>, Block<'a>, Option<Block<'a>>),
    For(Name<'a>, Box<Expr<'a>>, Block<'a>),
    While(Box<Expr<'a>>, Block<'a>),
    Block(Block<'a>),
}

pub type Expr<'a> = Spanned<RawExpr<'a>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawExpr<'a> {
    Literal(Spanned<Literal<'a>>),
    Var(Name<'a>),
    // Block(Block<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Spanned<BinOp>, Box<Expr<'a>>),
    UnaryOp(Spanned<UnaryOp>, Box<Expr<'a>>),
    FieldAccess(Box<Expr<'a>>, #[serde(borrow)] Name<'a>),
    ArrayAccess(Box<Expr<'a>>, Box<Expr<'a>>),
    Array(Vec<Expr<'a>>),
    Map(Map<'a>),
}

// This preserves duplicated names, which are handled later
type Map<'a> = Vec<(Name<'a>, Expr<'a>)>;

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
