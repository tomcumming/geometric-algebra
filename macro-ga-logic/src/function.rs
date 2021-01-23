use crate::{Expr, MVType};

pub struct Function {
    args: Vec<(String, MVType)>,
    body: Expr,
}

impl Function {
    pub fn args(&self) -> &Vec<(String, MVType)> {
        &self.args
    }

    pub fn body(&self) -> &Expr {
        &self.body
    }

    pub fn new(args: Vec<(String, MVType)>, body: Expr) -> Result<Function, String> {
        // TODO check arg names are valid and unique
        // TODO check expression in valid in ctx

        Ok(Function { args, body })
    }
}
