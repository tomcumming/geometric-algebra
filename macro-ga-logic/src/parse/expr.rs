use std::str::FromStr;

use proc_macro2::{Delimiter, TokenTree};

use crate::parse::element::try_parse_element;
use crate::parse::Tokens;
use crate::Expr;

fn parse_constant(literal: String) -> Result<Expr, String> {
    let parsed =
        isize::from_str(&literal).map_err(|_e| format!("Could not parse literal '{}'", literal))?;
    Ok(Expr::Constant(parsed))
}

fn parse_ident(name: String) -> Expr {
    try_parse_element(&name)
        .map(Expr::Element)
        .unwrap_or(Expr::Symbol(name))
}

pub fn parse_operand(tokens: &mut Tokens) -> Result<Expr, String> {
    let next_token = tokens.next().ok_or("Unexpected end of expression")?;

    match next_token {
        TokenTree::Literal(l) => parse_constant(l.to_string()),
        TokenTree::Ident(i) => Ok(parse_ident(i.to_string())),
        TokenTree::Punct(p) if p.as_char() == '-' => {
            let e = parse_operand(tokens)?;
            Ok(Expr::Negate(Box::new(e)))
        }
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
            let mut tokens = g.stream().into_iter().peekable();
            Ok(Expr::Brackets(parse_expression(&mut tokens)?.into()))
        }
        token => Err(format!("Unexpected token in operand '{}'", token)),
    }
}

pub fn parse_expression(tokens: &mut Tokens) -> Result<Expr, String> {
    let lhs = parse_operand(tokens)?;

    match tokens.peek() {
        Some(TokenTree::Punct(p)) if p.as_char() == '+' => parse_add_sub(tokens, Expr::Add, lhs),
        Some(TokenTree::Punct(p)) if p.as_char() == '-' => parse_add_sub(tokens, Expr::Sub, lhs),
        Some(TokenTree::Punct(p)) if p.as_char() == '*' => parse_mul_div(tokens, Expr::Mul, lhs),
        Some(TokenTree::Punct(p)) if p.as_char() == '/' => parse_mul_div(tokens, Expr::Div, lhs),
        _ => Ok(lhs),
    }
}

fn parse_add_sub(
    tokens: &mut Tokens,
    constructor: fn(Box<Expr>, Box<Expr>) -> Expr,
    lhs: Expr,
) -> Result<Expr, String> {
    tokens.next().expect("Expected to skip plus symbol");

    fn add_left(constructor: fn(Box<Expr>, Box<Expr>) -> Expr, lhs: Expr, e: Expr) -> Expr {
        match e {
            Expr::Add(e_lhs, e_rhs) => {
                Expr::Add(Box::new(add_left(constructor, lhs, *e_lhs)), e_rhs)
            }
            Expr::Sub(e_lhs, e_rhs) => {
                Expr::Sub(Box::new(add_left(constructor, lhs, *e_lhs)), e_rhs)
            }
            e => constructor(Box::new(lhs), Box::new(e)),
        }
    }

    let rhs = parse_expression(tokens)?;
    Ok(add_left(constructor, lhs, rhs))
}

fn parse_mul_div(
    tokens: &mut Tokens,
    constructor: fn(Box<Expr>, Box<Expr>) -> Expr,
    lhs: Expr,
) -> Result<Expr, String> {
    tokens.next().expect("Expected to skip mul symbol");

    fn mul_left(constructor: fn(Box<Expr>, Box<Expr>) -> Expr, lhs: Expr, e: Expr) -> Expr {
        match e {
            Expr::Add(e_lhs, e_rhs) => {
                Expr::Add(Box::new(mul_left(constructor, lhs, *e_lhs)), e_rhs)
            }
            Expr::Sub(e_lhs, e_rhs) => {
                Expr::Sub(Box::new(mul_left(constructor, lhs, *e_lhs)), e_rhs)
            }
            Expr::Mul(e_lhs, e_rhs) => {
                Expr::Mul(Box::new(mul_left(constructor, lhs, *e_lhs)), e_rhs)
            }
            Expr::Div(e_lhs, e_rhs) => {
                Expr::Div(Box::new(mul_left(constructor, lhs, *e_lhs)), e_rhs)
            }
            e => constructor(Box::new(lhs), Box::new(e)),
        }
    }

    let rhs = parse_expression(tokens)?;
    Ok(mul_left(constructor, lhs, rhs))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    use super::*;

    #[test]
    fn test_parse_single_eos() {
        let mut tokens = TokenStream::new().into_iter().peekable();
        match parse_operand(&mut tokens) {
            Err(_e) => {}
            Ok(_l) => panic!("Should have failed"),
        }
    }

    #[test]
    fn test_parse_constants() -> Result<(), String> {
        let mut tokens = TokenStream::from_str("1 12 123")
            .unwrap()
            .into_iter()
            .peekable();

        for expected in [1, 12, 123].iter() {
            match parse_operand(&mut tokens)?.into() {
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
        let mut tokens = TokenStream::from_str("你好   World")
            .unwrap()
            .into_iter()
            .peekable();

        for expected in ["你好", "World"].iter() {
            match parse_operand(&mut tokens).unwrap().into() {
                Expr::Symbol(s) => {
                    assert_eq!(s.as_str(), *expected);
                }
                _ => panic!("Parsed something other than constant"),
            };
        }
    }

    #[test]
    fn test_parse_negated_number() {
        let mut tokens = TokenStream::from_str("-123")
            .unwrap()
            .into_iter()
            .peekable();

        let e: Expr = parse_operand(&mut tokens).unwrap().into();
        assert_eq!(e, Expr::Negate(Box::new(Expr::Constant(123))));
    }

    #[test]
    fn test_parse_base_elements() {
        let examples = [
            ("e0", Expr::Element(vec![0])),
            ("e1", Expr::Element(vec![1])),
            ("e2e1e0", Expr::Element(vec![2, 1, 0])),
        ];

        for (src, expected) in examples.iter() {
            let mut tokens = TokenStream::from_str(src).unwrap().into_iter().peekable();
            let e: Expr = parse_expression(&mut tokens).unwrap().into();
            assert_eq!(&e, expected);
        }
    }

    #[test]
    fn test_parse_simple_addition() {
        let mut tokens = TokenStream::from_str("1 + 2 +   3")
            .unwrap()
            .into_iter()
            .peekable();

        let e: Expr = parse_expression(&mut tokens).unwrap().into();
        assert_eq!(
            e,
            Expr::Add(
                Box::new(Expr::Add(
                    Box::new(Expr::Constant(1)),
                    Box::new(Expr::Constant(2))
                )),
                Box::new(Expr::Constant(3)),
            )
        );
    }

    #[test]
    fn test_parse_addition_brackets() {
        let mut tokens = TokenStream::from_str("1 + (2 + 3)")
            .unwrap()
            .into_iter()
            .peekable();

        let e: Expr = parse_expression(&mut tokens).unwrap().into();
        assert_eq!(
            e,
            Expr::Add(
                Box::new(Expr::Constant(1)),
                Box::new(Expr::Brackets(Box::new(Expr::Add(
                    Box::new(Expr::Constant(2)),
                    Box::new(Expr::Constant(3))
                ))))
            )
        );
    }

    #[test]
    fn test_parse_simple_subtraction() {
        let mut tokens = TokenStream::from_str("1   - 2 - 3")
            .unwrap()
            .into_iter()
            .peekable();

        let e: Expr = parse_expression(&mut tokens).unwrap().into();
        assert_eq!(
            e,
            Expr::Sub(
                Box::new(Expr::Sub(
                    Box::new(Expr::Constant(1)),
                    Box::new(Expr::Constant(2))
                )),
                Box::new(Expr::Constant(3)),
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
                        Box::new(Expr::Constant(1)),
                        Box::new(Expr::Constant(2)),
                    )),
                    Box::new(Expr::Constant(3)),
                ),
            ),
            (
                "1 * (2 + 3)",
                Expr::Mul(
                    Box::new(Expr::Constant(1)),
                    Box::new(Expr::Brackets(Box::new(Expr::Add(
                        Box::new(Expr::Constant(2)),
                        Box::new(Expr::Constant(3)),
                    )))),
                ),
            ),
            (
                "1 + 2 * 3",
                Expr::Add(
                    Box::new(Expr::Constant(1)),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Constant(2)),
                        Box::new(Expr::Constant(3)),
                    )),
                ),
            ),
            (
                "(1 + 2) * 3",
                Expr::Mul(
                    Box::new(Expr::Brackets(Box::new(Expr::Add(
                        Box::new(Expr::Constant(1)),
                        Box::new(Expr::Constant(2)),
                    )))),
                    Box::new(Expr::Constant(3)),
                ),
            ),
        ];

        for (src, expected) in examples.iter() {
            let mut tokens = TokenStream::from_str(src).unwrap().into_iter().peekable();
            let e: Expr = parse_expression(&mut tokens).unwrap().into();
            assert_eq!(&e, expected);
        }
    }

    #[test]
    fn test_parse_simple_division() {
        let mut tokens = TokenStream::from_str("1 / 2 / 3")
            .unwrap()
            .into_iter()
            .peekable();

        let e: Expr = parse_expression(&mut tokens).unwrap().into();
        assert_eq!(
            e,
            Expr::Div(
                Box::new(Expr::Div(
                    Box::new(Expr::Constant(1)),
                    Box::new(Expr::Constant(2))
                )),
                Box::new(Expr::Constant(3)),
            )
        );
    }
}
