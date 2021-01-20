pub enum Expr {
    Element(usize),
    Symbol(String),
    Constant(f32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
