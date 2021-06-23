use std::fmt::Write;

use diagnostics::span::Spanned;
use serde::{Deserialize, Serialize};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/src/grammar.rs"
);

// mod grammar; // synthesized by LALRPOP

pub use grammar::ProgramParser;

// use codespan_reporting::diagnostic::Label;
// use serde::{Deserialize, Serialize};

// use crate::{diagnostics, env, value};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item<'a> {
    Function(Viz, #[serde(borrow)] Spanned<Function<'a>>),
    Import(Name<'a>),
    Const(Viz, Name<'a>, Expr<'a>),
}

pub type Viz = Spanned<RawViz>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawViz {
    Pub,
    // Default
    Private,
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

pub type Path<'a> = Vec<Name<'a>>;

pub type Expr<'a> = Spanned<RawExpr<'a>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawExpr<'a> {
    Literal(Spanned<Literal<'a>>),
    Var(Path<'a>),
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

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
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
    Null,
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

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            UnaryOp::Not => '!',
            UnaryOp::Minus => '-',
        })
    }
}

//===----------------------------------------------------------------------===//
//
// Span stuff
//
//===----------------------------------------------------------------------===//
