use std::collections::BTreeSet;

use proc_macro2::TokenTree;

use crate::parse::element::try_parse_element;
use crate::parse::Tokens;
use crate::{Element, MVType};

pub fn parse_element(tokens: &mut Tokens) -> Result<Element, String> {
    let token = tokens
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

pub fn parse_element_list(tokens: &mut Tokens) -> Result<Vec<Element>, String> {
    let mut elems = vec![parse_element(tokens)?];

    loop {
        let next = tokens.peek();
        match next {
            Some(TokenTree::Punct(p)) if p.as_char() == '+' => {
                tokens.next().expect("Just peeked a plus token");
                elems.push(parse_element(tokens)?);
            }
            _ => break,
        }
    }

    Ok(elems)
}

pub fn parse_type(tokens: &mut Tokens) -> Result<MVType, String> {
    let elems = parse_element_list(tokens)?;

    let ordered = elems.windows(2).all(|els| match els {
        [left, right] => left < right,
        _ => unreachable!("Window size 2"),
    });

    if ordered {
        Ok(MVType(elems.into_iter().collect()))
    } else {
        let correct_order = elems.clone().sort();

        Err(format!(
            "Elements in type must be in order and unique: '{:?}' => '{:?}'",
            elems, correct_order
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    use symbolic_ga::basis::Vector;

    use super::*;

    #[test]
    fn test_parse_element_list() {
        let mut tokens = TokenStream::from_str("1 + e1 + e1e2e3|")
            .unwrap()
            .into_iter()
            .peekable();

        assert_eq!(
            parse_element_list(&mut tokens).unwrap(),
            vec![
                Element(BTreeSet::new()),
                Element(vec![1].into_iter().map(Vector).collect()),
                Element(vec![1, 2, 3].into_iter().map(Vector).collect()),
            ]
        )
    }

    #[test]
    fn test_parse_element_list_wrong_order() {
        let mut tokens = TokenStream::from_str("e2e1")
            .unwrap()
            .into_iter()
            .peekable();
        assert!(parse_element_list(&mut tokens).is_err())
    }

    #[test]
    fn test_parse_element_list_duplicates() {
        let mut tokens = TokenStream::from_str("e1e2e2")
            .unwrap()
            .into_iter()
            .peekable();
        assert!(parse_element_list(&mut tokens).is_err())
    }
}
