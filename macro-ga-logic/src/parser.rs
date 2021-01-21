use std::iter::Peekable;
use std::str::FromStr;

use proc_macro2::token_stream::{self};
use proc_macro2::{TokenStream, TokenTree};

use crate::expr::Expr;

pub struct Parser {
    tokens: Peekable<token_stream::IntoIter>,
}

impl Parser {
    pub fn from_tokens(token_stream: TokenStream) -> Parser {
        Parser {
            tokens: token_stream.into_iter().peekable(),
        }
    }
}

impl Parser {
    pub fn parse_operand(&mut self) -> Result<Expr, String> {
        let next_token = self.tokens.next().ok_or("Unexpected end of expression")?;

        match next_token {
            TokenTree::Literal(l) => parse_constant(l.to_string()),
            TokenTree::Ident(i) => Ok(parse_ident(i.to_string())),
            token => Err(format!("Unexpected token '{}'", token)),
        }
    }
}

fn parse_constant(literal: String) -> Result<Expr, String> {
    let parsed =
        f32::from_str(&literal).map_err(|_e| format!("Could not parse literal '{}'", literal))?;
    Ok(Expr::Constant(parsed))
}

fn parse_ident(name: String) -> Expr {
    Expr::Symbol(name)
}

#[cfg(test)]
mod tests {
    use proc_macro2::Literal;

    use super::*;

    #[test]
    fn test_parse_single_eos() {
        let mut parser = Parser::from_tokens(TokenStream::new());
        match parser.parse_operand() {
            Err(_e) => {}
            Ok(_l) => panic!("Should have failed"),
        }
    }

    #[test]
    fn test_parse_constants() -> Result<(), String> {
        let mut parser = Parser::from_tokens(TokenStream::from_str("1.23 123").unwrap());

        for expected in [1.23, 123.0].iter() {
            match parser.parse_operand()? {
                Expr::Constant(c) => {
                    assert_eq!(c, *expected);
                }
                _ => return Err("Parsed something other than constant".to_string()),
            };
        }

        Ok(())
    }

    #[test]
    fn test_parse_symbols() -> Result<(), String> {
        let mut parser = Parser::from_tokens(TokenStream::from_str("你好 World").unwrap());

        for expected in ["你好", "World"].iter() {
            match parser.parse_operand()? {
                Expr::Symbol(s) => {
                    assert_eq!(s.as_str(), *expected);
                }
                _ => return Err("Parsed something other than constant".to_string()),
            };
        }

        Ok(())
    }
}
