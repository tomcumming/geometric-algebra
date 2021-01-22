#[derive(Debug, PartialEq)]
pub enum Expr {
    Brackets(Box<Expr>),
    Vector(usize),
    Symbol(String),
    Constant(f32),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
