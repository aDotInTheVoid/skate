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
    pub body: Body,
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
pub struct Body;
