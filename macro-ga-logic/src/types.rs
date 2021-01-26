use proc_macro2::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenTree};

use crate::tokens::tokenstream_push;
use crate::{CodeBasis, Element, MVType};

pub fn element_type_name(basis: &CodeBasis, Element(vs): &Element) -> String {
    if vs.is_empty() {
        basis.scalar.to_string()
    } else {
        let mut name = String::new();
        for v in vs {
            name += "E";
            name += &v.0.to_string();
        }
        name
    }
}

pub fn element_term_name(Element(vs): &Element) -> String {
    if vs.is_empty() {
        "1".to_string()
    } else {
        let mut name = String::new();
        for v in vs {
            name += "e";
            name += &v.0.to_string();
        }
        name
    }
}

pub fn type_signiture(basis: &CodeBasis, MVType(es): &MVType) -> TokenStream {
    let mut tokens = TokenStream::new();

    for e in es.iter() {
        if !tokens.is_empty() {
            tokenstream_push(&mut tokens, Punct::new(',', Spacing::Alone).into());
        }

        tokenstream_push(
            &mut tokens,
            Ident::new(&element_type_name(basis, e), Span::call_site()).into(),
        );
    }

    if es.len() > 1 {
        std::iter::once(TokenTree::from(Group::new(Delimiter::Parenthesis, tokens))).collect()
    } else {
        tokens
    }
}
