use std::iter::Peekable;
use std::str::FromStr;

use proc_macro2::token_stream::{self};
use proc_macro2::{TokenStream, TokenTree};

use crate::expr::Expr;

struct Parser {
    tokens: Peekable<token_stream::IntoIter>,
}

impl Parser {
    fn from_tokens(token_stream: TokenStream) -> Parser {
        Parser {
            tokens: token_stream.into_iter().peekable(),
        }
    }
}

impl Parser {
    pub fn parse_single(&mut self) -> Result<Expr, String> {
        let next_token = self.tokens.next().ok_or("Unexpected end of expression")?;

        match next_token {
            TokenTree::Literal(l) => parse_constant(l.to_string()),
            _ => todo!("rest of parser"),
        }
    }
}

fn parse_constant(literal: String) -> Result<Expr, String> {
    let parsed =
        f32::from_str(&literal).map_err(|_e| format!("Could not parse literal '{}'", literal))?;
    Ok(Expr::Constant(parsed))
}

#[cfg(test)]
mod tests {
    use proc_macro2::Literal;

    use super::*;

    #[test]
    fn test_parse_single_eos() {
        let mut parser = Parser::from_tokens(TokenStream::new());
        match parser.parse_single() {
            Err(_e) => {}
            Ok(_l) => panic!("Should have failed"),
        }
    }

    #[test]
    fn test_parse_single_constants() -> Result<(), String> {
        let tokens: Vec<TokenTree> = vec![
            Literal::f32_unsuffixed(1.23).into(),
            Literal::usize_unsuffixed(123).into(),
        ];
        let mut parser = Parser::from_tokens(tokens.into_iter().collect());

        for expected in [1.23, 123.0].iter() {
            match parser.parse_single()? {
                Expr::Constant(c) => {
                    assert_eq!(c, *expected);
                }
                _ => return Err("Parsed something other than constant".to_string()),
            };
        }

        Ok(())
    }
}
