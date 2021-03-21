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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function<'a> {
    pub name: &'a str,
    pub args: Vec<Arg<'a>>,
    pub ret: Option<Type>,

    pub body: Vec<Stmt<'a>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr<'a> {
    Var(&'a str),
    Block(Vec<Stmt<'a>>),
}
