use proc_macro2::TokenTree;

use crate::lambda::Lambda;
use crate::parse::expr::parse_expression;
use crate::parse::mvtype::parse_type;
use crate::parse::Tokens;
use crate::MVType;

pub fn parse_lambda(tokens: &mut Tokens) -> Result<Lambda, String> {
    match tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '|' => {}
        token => {
            let token = token
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOS".to_string());
            return Err(format!("Expected '|' before args, got: {}", token));
        }
    };

    let args = parse_args(tokens)?;
    let body = parse_expression(tokens)?;

    Lambda::new(args, body)
}

fn parse_args(tokens: &mut Tokens) -> Result<Vec<(String, MVType)>, String> {
    let id = match tokens.next() {
        Some(TokenTree::Ident(i)) => i.to_string(),
        token => {
            let token = token
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOS".to_string());
            return Err(format!("Expected arg name, got: {}", token));
        }
    };

    match tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ':' => {}
        token => {
            let token = token
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOS".to_string());
            return Err(format!("Expected ':' after arg name, got: {}", token));
        }
    };

    let mv_type = parse_type(tokens)?;

    let arg = (id, mv_type);

    match tokens.peek() {
        Some(TokenTree::Punct(p)) if p.as_char() == '|' => {
            tokens.next().expect("Peeked end of args");
            Ok(vec![arg])
        }
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
            tokens.next().expect("Peeked comma");
            parse_args(tokens).map(|rest| std::iter::once(arg).chain(rest.into_iter()).collect())
        }
        token => {
            let token = token
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOS".to_string());
            Err(format!("Expected ',' or '|' after args, got: {}", token))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    use symbolic_ga::basis::Vector;

    use super::*;
    use crate::{Element, Expr};

    #[test]
    fn test_parse_simple_function() {
        let mut tokens = TokenStream::from_str("|a: 1 + e1e2, b: e1 + e2| a * b")
            .unwrap()
            .into_iter()
            .peekable();

        let expected_args = vec![
            (
                "a".to_string(),
                MVType(
                    vec![
                        Element(BTreeSet::new()),
                        Element(vec![1, 2].into_iter().map(Vector).collect()),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ),
            (
                "b".to_string(),
                MVType(
                    vec![
                        Element(std::iter::once(Vector(1)).collect()),
                        Element(std::iter::once(Vector(2)).collect()),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ),
        ];

        let expected_body = Expr::Mul(
            Box::new(Expr::Symbol("a".to_string())),
            Box::new(Expr::Symbol("b".to_string())),
        );

        let f = parse_lambda(&mut tokens).unwrap();

        assert_eq!(f.args(), &expected_args);
        assert_eq!(f.body(), &expected_body);
    }
}
