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
pub struct Function(pub String, pub Args, pub Body);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Args(pub Vec<Arg>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg(pub String, pub Type);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Bool,
    Int,
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body;
