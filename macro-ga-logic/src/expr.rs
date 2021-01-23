use crate::parse::element::VectorIndex;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Brackets(Box<Expr>),
    Element(Vec<VectorIndex>),
    Symbol(String),
    Constant(f32),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
