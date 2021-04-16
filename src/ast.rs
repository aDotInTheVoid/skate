use serde::{Deserialize, Serialize};

// TODO: Clean up "outer" vs inner spans
// Eg an Expr has its own assocd span, but a BinOp doesnt

pub type Block<'a> = Spanned<Vec<Stmt<'a>>>;
pub type Program<'a> = Vec<Item<'a>>;
pub type Span = (usize, usize);

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
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
    pub name: Spanned<&'a str>,
    #[serde(borrow)]
    pub args: Spanned<Vec<Arg<'a>>>,
    pub ret: Option<Spanned<Type>>,
    #[serde(borrow)]
    pub body: Block<'a>,
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

pub type Expr<'a> = Spanned<RawExpr<'a>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawExpr<'a> {
    Literal(Spanned<Literal<'a>>),
    Var(Spanned<&'a str>),
    Block(Block<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Spanned<BinOp>, Box<Expr<'a>>),
    UnaryOp(Spanned<UnaryOp>, Box<Expr<'a>>),
    FieldAccess(Box<Expr<'a>>, Spanned<&'a str>),
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
