pub mod expr;
pub mod function;
pub mod parse;
mod tokens;
pub mod types;

use std::collections::BTreeSet;

use symbolic_ga::basis::{Basis, Vector};
use symbolic_ga::element::Element;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MVType(pub BTreeSet<Element>);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Brackets(Box<Expr>),
    Element(Vec<Vector>),
    Symbol(String),
    Constant(isize),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

pub struct CodeBasis {
    pub basis: Basis,
    pub scalar: String,
}
