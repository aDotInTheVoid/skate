use serde::{Deserialize, Serialize};

pub type Block<'a> = Vec<Stmt<'a>>;
pub type Program<'a> = Vec<Item<'a>>;
pub type Span = (usize, usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item<'a> {
    Function(#[serde(borrow)] Spanned<Function<'a>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function<'a> {
    #[serde(borrow)]
    pub name: Spanned<&'a str>,
    #[serde(borrow)]
    pub args: Spanned<Vec<Arg<'a>>>,
    pub ret: Option<Spanned<Type>>,
    #[serde(borrow)]
    pub body: Spanned<Block<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg<'a> {
    #[serde(borrow)]
    pub name: Spanned<&'a str>,
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
    Let(#[serde(borrow)] Spanned<&'a str>, Expr<'a>),
    Expr(Expr<'a>),
    Print(Expr<'a>),
    Return(Expr<'a>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Var(&'a str),
    Block(Block<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, BinOp, Box<Expr<'a>>),
    UnaryOp(UnaryOp, Box<Expr<'a>>),
    FieldAccess(Box<Expr<'a>>, &'a str),
    ArrayAccess(Box<Expr<'a>>, Box<Expr<'a>>),
    // Test, truecase, falsecase
    If(Box<Expr<'a>>, Block<'a>, Option<Block<'a>>),
    For(&'a str, Box<Expr<'a>>, Block<'a>),
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
