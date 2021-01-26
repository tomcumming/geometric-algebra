use std::collections::BTreeMap;

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::expr::{mv_as_code, simplify_expr};
use crate::tokens::tokenstream_push;
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

    pub fn as_code(&self, basis: &CodeBasis) -> Result<TokenStream, String> {
        let mut tokens = TokenStream::new();
        tokenstream_push(&mut tokens, Punct::new('|', Spacing::Alone).into());
        tokens.extend(args_as_code(basis, &self.args));
        tokenstream_push(&mut tokens, Punct::new('|', Spacing::Alone).into());

        let sym_types: BTreeMap<String, MVType> = self.args.iter().cloned().collect();

        let mv = simplify_expr(basis, &sym_types, &self.body)?;
        tokens.extend(mv_as_code(basis, &mv));

        Ok(tokens)
    }
}

fn args_as_code(basis: &CodeBasis, args: &[(String, MVType)]) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (name, mv_type) in args {
        tokens.extend(arg_as_code(basis, name, mv_type));
        tokenstream_push(&mut tokens, Punct::new(',', Spacing::Alone).into());
    }

    tokens
}

fn arg_as_code(basis: &CodeBasis, name: &str, mv_type: &MVType) -> TokenStream {
    let mut tokens = TokenStream::new();

    let mut pattern_tokens = TokenStream::new();
    for e in mv_type.0.iter() {
        if !pattern_tokens.is_empty() {
            tokenstream_push(&mut pattern_tokens, Punct::new(',', Spacing::Alone).into());
        }

        let term_name = format!("{}_{}", name, element_term_name(e));
        if e.0.is_empty() {
            tokenstream_push(
                &mut pattern_tokens,
                Ident::new(&term_name, Span::call_site()).into(),
            );
        } else {
            // This is just a constructor pattern like 'E1(a_e1)`
            tokenstream_push(
                &mut pattern_tokens,
                Ident::new(&element_type_name(basis, e), Span::call_site()).into(),
            );
            let term_name_token: TokenTree = Ident::new(&term_name, Span::call_site()).into();
            tokenstream_push(
                &mut pattern_tokens,
                TokenTree::from(Group::new(
                    Delimiter::Parenthesis,
                    std::iter::once(term_name_token).collect(),
                )),
            );
        }
    }

    if mv_type.0.len() > 1 {
        tokenstream_push(
            &mut tokens,
            TokenTree::from(Group::new(Delimiter::Parenthesis, pattern_tokens)),
        );
    } else {
        tokens.extend(pattern_tokens);
    }

    tokenstream_push(&mut tokens, Punct::new(':', Spacing::Alone).into());
    tokens.extend(type_signiture(basis, mv_type));

    tokens
}
