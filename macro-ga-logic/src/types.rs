use proc_macro2::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenTree};

use crate::{CodeBasis, Element, MVType};

pub fn type_signiture(basis: &CodeBasis, MVType(es): &MVType) -> TokenStream {
    let mut tokens: Vec<TokenTree> = Vec::new();

    for Element(vs) in es.iter() {
        if !tokens.is_empty() {
            tokens.push(Punct::new(',', Spacing::Alone).into())
        }

        if vs.is_empty() {
            tokens.push(Ident::new(&basis.scalar, Span::call_site()).into());
        } else {
            let mut name = String::new();
            for v in vs {
                name += "E";
                name += &v.to_string();
            }
            tokens.push(Ident::new(&name, Span::call_site()).into());
        }
    }

    std::iter::once(TokenTree::from(Group::new(
        Delimiter::Parenthesis,
        tokens.into_iter().collect(),
    )))
    .collect()
}
