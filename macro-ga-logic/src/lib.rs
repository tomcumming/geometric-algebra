pub mod function;
pub mod parse;

use std::collections::BTreeSet;

pub type VectorIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element(pub BTreeSet<VectorIndex>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MVType(pub BTreeSet<Element>);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Brackets(Box<Expr>),
    Element(Vec<VectorIndex>),
    Symbol(String),
    Constant(isize),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
