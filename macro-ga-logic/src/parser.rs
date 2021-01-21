use std::iter::Peekable;
use std::str::FromStr;

use proc_macro2::token_stream::{self};
use proc_macro2::{Delimiter, TokenStream, TokenTree};

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

#[derive(Debug, PartialEq)]
pub enum ParsedExpr {
    Brackets(Expr),
    Naked(Expr),
}

impl From<ParsedExpr> for Expr {
    fn from(pe: ParsedExpr) -> Expr {
        match pe {
            ParsedExpr::Brackets(e) => e,
            ParsedExpr::Naked(e) => e,
        }
    }
}

impl Parser {
    pub fn parse_operand(&mut self) -> Result<ParsedExpr, String> {
        let next_token = self.tokens.next().ok_or("Unexpected end of expression")?;

        match next_token {
            TokenTree::Literal(l) => parse_constant(l.to_string()).map(ParsedExpr::Naked),
            TokenTree::Ident(i) => Ok(ParsedExpr::Naked(parse_ident(i.to_string()))),
            TokenTree::Punct(p) if p.as_char() == '-' => {
                let e = self.parse_operand()?;
                Ok(ParsedExpr::Naked(Expr::Mul(
                    Box::new(Expr::Constant(-1.0)),
                    Box::new(e.into()),
                )))
            }
            TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
                let mut sub_parser = Parser::from_tokens(g.stream());
                Ok(ParsedExpr::Brackets(sub_parser.parse_expression()?.into()))
            }
            token => Err(format!("Unexpected token in operand '{}'", token)),
        }
    }

    pub fn parse_expression(&mut self) -> Result<ParsedExpr, String> {
        let lhs = self.parse_operand()?;

        match self.tokens.peek() {
            None => Ok(lhs),
            Some(token) => match token {
                TokenTree::Punct(p) if p.as_char() == '+' => {
                    self.tokens.next(); // skip the + token
                    let rhs: Expr = self.parse_expression()?.into();
                    Ok(ParsedExpr::Naked(Expr::Add(
                        Box::new(lhs.into()),
                        Box::new(rhs),
                    )))
                }
                token => Err(format!("Unexpected token in expression '{}'", token)),
            },
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
            match parser.parse_operand()?.into() {
                Expr::Constant(c) => {
                    assert_eq!(c, *expected);
                }
                _ => return Err("Parsed something other than constant".to_string()),
            };
        }

        Ok(())
    }

    #[test]
    fn test_parse_symbols() {
        let mut parser = Parser::from_tokens(TokenStream::from_str("你好   World").unwrap());

        for expected in ["你好", "World"].iter() {
            match parser.parse_operand().unwrap().into() {
                Expr::Symbol(s) => {
                    assert_eq!(s.as_str(), *expected);
                }
                _ => panic!("Parsed something other than constant"),
            };
        }
    }

    #[test]
    fn test_parse_negated_number() {
        let mut parser = Parser::from_tokens(TokenStream::from_str("-123").unwrap());

        let e: Expr = parser.parse_operand().unwrap().into();
        assert_eq!(
            e,
            Expr::Mul(
                Box::new(Expr::Constant(-1.0)),
                Box::new(Expr::Constant(123.0))
            )
        );
    }

    #[test]
    fn test_parse_simple_addition() {
        let mut parser = Parser::from_tokens(TokenStream::from_str("1 + 2 +   3").unwrap());

        let e: Expr = parser.parse_expression().unwrap().into();
        assert_eq!(
            e,
            Expr::Add(
                Box::new(Expr::Constant(1.0)),
                Box::new(Expr::Add(
                    Box::new(Expr::Constant(2.0)),
                    Box::new(Expr::Constant(3.0))
                ))
            )
        );
    }

    #[test]
    fn test_parse_brackets() {
        let mut parser = Parser::from_tokens(TokenStream::from_str("(1 + 2) + 3").unwrap());

        let e: Expr = parser.parse_expression().unwrap().into();
        assert_eq!(
            e,
            Expr::Add(
                Box::new(Expr::Add(
                    Box::new(Expr::Constant(1.0)),
                    Box::new(Expr::Constant(2.0))
                )),
                Box::new(Expr::Constant(3.0))
            )
        );
    }

    #[test]
    fn test_parse_multiplication() {
        let examples = [
            (
                "1 * 2 + 3",
                Expr::Add(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Constant(1.0)),
                        Box::new(Expr::Constant(2.0)),
                    )),
                    Box::new(Expr::Constant(3.0)),
                ),
            ),
            (
                "1 * (2 + 3)",
                Expr::Mul(
                    Box::new(Expr::Constant(1.0)),
                    Box::new(Expr::Add(
                        Box::new(Expr::Constant(2.0)),
                        Box::new(Expr::Constant(3.0)),
                    )),
                ),
            ),
            (
                "1 + 2 * 3",
                Expr::Add(
                    Box::new(Expr::Constant(1.0)),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Constant(2.0)),
                        Box::new(Expr::Constant(3.0)),
                    )),
                ),
            ),
            (
                "(1 + 2) * 3",
                Expr::Mul(
                    Box::new(Expr::Add(
                        Box::new(Expr::Constant(1.0)),
                        Box::new(Expr::Constant(2.0)),
                    )),
                    Box::new(Expr::Constant(3.0)),
                ),
            ),
        ];

        for (src, expected) in examples.iter() {
            let mut parser = Parser::from_tokens(TokenStream::from_str(src).unwrap());
            let e: Expr = parser.parse_expression().unwrap().into();
            assert_eq!(&e, expected);
        }
    }
}