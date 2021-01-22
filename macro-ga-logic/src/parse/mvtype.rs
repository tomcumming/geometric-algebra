use std::collections::BTreeSet;
use std::iter::Peekable;

use proc_macro2::token_stream::{self};
use proc_macro2::{TokenStream, TokenTree};

use crate::parse::element::{try_parse_element, Element};

pub struct Parser {
    tokens: Peekable<token_stream::IntoIter>,
}

impl Parser {
    pub fn from_token_stream(token_stream: TokenStream) -> Parser {
        Parser {
            tokens: token_stream.into_iter().peekable(),
        }
    }
}

impl Parser {
    pub fn parse_element(&mut self) -> Result<Element, String> {
        let token = self
            .tokens
            .next()
            .ok_or_else(|| "Expected another element in type".to_string())?;

        match token {
            TokenTree::Literal(l) if &l.to_string() == "1" => Ok(Element(BTreeSet::new())),
            TokenTree::Ident(i) => {
                let element_name = try_parse_element(&i.to_string())
                    .ok_or(format!("While parsing element name, got '{}'", i))?;

                let ordered = element_name.windows(2).all(|els| match els {
                    [left, right] => left < right,
                    _ => unreachable!("Window size 2"),
                });

                if ordered {
                    Ok(Element(element_name.into_iter().collect()))
                } else {
                    Err(format!(
                        "Element vectors must be unique and ordered: '{}'",
                        i
                    ))
                }
            }
            token => Err(format!(
                "Expected an element name, got '{}'",
                token.to_string()
            )),
        }
    }

    pub fn parse_element_list(&mut self) -> Result<Vec<Element>, String> {
        let mut elems = vec![self.parse_element()?];

        loop {
            let next = self.tokens.peek();
            match next {
                Some(TokenTree::Punct(p)) if p.as_char() == '+' => {
                    self.tokens.next().expect("Just peeked a plus token");
                    elems.push(self.parse_element()?);
                }
                _ => break,
            }
        }

        Ok(elems)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_parse_element_list() {
        let mut parser =
            Parser::from_token_stream(TokenStream::from_str("1 + e1 + e1e2e3|").unwrap());

        assert_eq!(
            parser.parse_element_list().unwrap(),
            vec![
                Element(BTreeSet::new()),
                Element(vec![1].into_iter().collect()),
                Element(vec![1, 2, 3].into_iter().collect()),
            ]
        )
    }

    #[test]
    fn test_parse_element_list_wrong_order() {
        let mut parser = Parser::from_token_stream(TokenStream::from_str("e2e1").unwrap());
        assert!(parser.parse_element_list().is_err())
    }

    #[test]
    fn test_parse_element_list_duplicates() {
        let mut parser = Parser::from_token_stream(TokenStream::from_str("e1e2e2").unwrap());
        assert!(parser.parse_element_list().is_err())
    }
}
