#[derive(Debug, PartialEq)]
pub enum Expr {
    Vector(usize),
    Symbol(String),
    Constant(f32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
