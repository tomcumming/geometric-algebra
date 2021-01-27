use proc_macro2::TokenTree;

use crate::parse::expr::parse_expression;
use crate::parse::Tokens;
use crate::Expr;

pub fn parse_args(tokens: &mut Tokens) -> Result<Vec<Expr>, String> {
    let mut exprs = Vec::new();

    match tokens.next() {
        Some(TokenTree::Group(g)) => {
            let mut tokens = g.stream().into_iter().peekable();
            loop {
                exprs.push(parse_expression(&mut tokens)?);
                match tokens.next() {
                    None => return Ok(exprs),
                    Some(TokenTree::Punct(p)) if p.as_char() == ',' => continue,
                    token => {
                        let token = token
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "EOS".to_string());
                        return Err(format!("Expected ',' or ')' after args, got: {}", token));
                    }
                }
            }
        }
        token => {
            let token = token
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOS".to_string());
            Err(format!("Expected '(' before args, got: {}", token))
        }
    }
}
