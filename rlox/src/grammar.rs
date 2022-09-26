#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    UnaryExpr(UnaryOp, Box<Expr>),
    BinaryExpr(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Float(f64),
    Int(i32),
    True,
    False,
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Negative,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    Plus,
    Minus,
    Times,
    Divide,
    Assign,
}
