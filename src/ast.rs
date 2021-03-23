// struct Program(Vec<Item>);

// enum Item {
//     Function(Function),
// }

// struct Function {
//     name: String,
//     Body: Vec<Stmt>,
// }

// enum Stmt {
//     Let(Let),
// }

// struct Let {
//     left: String,
//     right: String,
// }

use serde::{Deserialize, Serialize};

type Block<'a> = Vec<Stmt<'a>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function<'a> {
    pub name: &'a str,
    pub args: Vec<Arg<'a>>,
    pub ret: Option<Type>,
    pub body: Block<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg<'a> {
    pub name: &'a str,
    pub ty: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Bool,
    Int,
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt<'a> {
    Let(&'a str, Expr<'a>),
    Expr(Expr<'a>),
    If(Expr<'a>, Block<'a>, Option<Block<'a>>),
    For(&'a str, Expr<'a>, Block<'a>),
    Print(Expr<'a>),
    While(Expr<'a>, Block<'a>),
    // This may need do {a} while b;
    // DoWhile(Block<'a>, Expr<'a>),
    Return(Expr<'a>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Var(&'a str),
    Block(Block<'a>),
    Call(Box<Expr<'a>>, Vec<Arg<'a>>),
    Binop(Box<Expr<'a>>, BinOp, Box<Expr<'a>>),
    Access(Box<Expr<'a>>, &'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Integer(i32),
}
