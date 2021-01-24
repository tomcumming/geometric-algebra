use proc_macro2::TokenStream;

use crate::types::type_signiture;
use crate::{CodeBasis, Expr, MVType};

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
        //      Must not conflict with suffixed names
        // TODO check expression in valid in ctx

        Ok(Function { args, body })
    }

    pub fn as_code(&self, basis: &CodeBasis) -> TokenStream {
        type_signiture(basis, &self.args[0].1)
    }
}
