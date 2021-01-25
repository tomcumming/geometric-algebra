use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::types::{element_term_name, element_type_name, type_signiture};
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
        let mut tokens: Vec<TokenTree> = Vec::new();
        tokens.push(Punct::new('|', Spacing::Alone).into());
        args_as_code(basis, &self.args, &mut tokens);
        tokens.push(Punct::new('|', Spacing::Alone).into());

        tokens.push(TokenTree::from(Group::new(Delimiter::Parenthesis, TokenStream::new())));

        tokens.into_iter().collect()
    }
}

fn args_as_code(basis: &CodeBasis, args: &[(String, MVType)], tokens: &mut Vec<TokenTree>) {
    for (name, mv_type) in args {
        arg_as_code(basis, name, mv_type, tokens);
        tokens.push(Punct::new(',', Spacing::Alone).into());
    }
}

fn arg_as_code(basis: &CodeBasis, name: &str, mv_type: &MVType, tokens: &mut Vec<TokenTree>) {
    let mut pattern_tokens: Vec<TokenTree> = Vec::new();
    for e in mv_type.0.iter() {
        if !pattern_tokens.is_empty() {
            pattern_tokens.push(Punct::new(',', Spacing::Alone).into())
        }

        let term_name = format!("{}_{}", name, element_term_name(e));
        if e.0.is_empty() {
            pattern_tokens.push(Ident::new(&term_name, Span::call_site()).into());
        } else {
            // This is just a constructor pattern like 'E1(a_e1)`
            pattern_tokens.push(Ident::new(&element_type_name(basis, e), Span::call_site()).into());
            let term_name_token: TokenTree = Ident::new(&term_name, Span::call_site()).into();
            pattern_tokens.push(TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                std::iter::once(term_name_token).collect(),
            )));
        }
    }

    if mv_type.0.len() > 1 {
        tokens.push(
            TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                pattern_tokens.into_iter().collect(),
            )),
        );
    } else {
        tokens.append(&mut pattern_tokens);
    }

    tokens.push(Punct::new(':', Spacing::Alone).into());
    tokens.append(&mut type_signiture(basis, mv_type).into_iter().collect());
}
