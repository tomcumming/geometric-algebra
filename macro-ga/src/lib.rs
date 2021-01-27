extern crate proc_macro;

mod function;

use std::collections::BTreeMap;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro::TokenTree;

static mut GLOBAL_BASIS: Option<BTreeMap<String, (usize, usize, usize)>> = None;

pub(crate) fn use_global_basis<R, F: FnOnce(&mut BTreeMap<String, (usize, usize, usize)>) -> R>(
    f: F,
) -> R {
    let mut basis = unsafe { GLOBAL_BASIS.take().unwrap_or_default() };

    let ret = f(&mut basis);

    unsafe {
        GLOBAL_BASIS = Some(basis);
    }

    ret
}

#[proc_macro]
pub fn define_basis(token_stream: TokenStream) -> TokenStream {
    let tokens: Vec<_> = token_stream.into_iter().collect();

    match tokens.as_slice() {
        [TokenTree::Ident(name), TokenTree::Punct(comma1), TokenTree::Literal(pos), TokenTree::Punct(comma2), TokenTree::Literal(neg), TokenTree::Punct(comma3), TokenTree::Literal(zero)]
            if [comma1, comma2, comma3].iter().all(|c| c.as_char() == ',') =>
        {
            let pos =
                usize::from_str(&pos.to_string()).expect("Could not parse positive basis count");
            let neg =
                usize::from_str(&neg.to_string()).expect("Could not parse negative basis count");
            let zero =
                usize::from_str(&zero.to_string()).expect("Could not parse zero basis count");

            // TODO check for redefinition with different value!
            use_global_basis(|basis| {
                basis.insert(name.to_string(), (pos, neg, zero));
            });
        }
        _tokens => {
            println!("{:?}", tokens);
            panic!("Expected something like 'define_basis!(G2, 2, 0, 0)");
        }
    };

    TokenStream::new()
}

#[proc_macro]
pub fn basis_types(token_stream: TokenStream) -> TokenStream {
    let mut tokens = token_stream.into_iter();

    let basis = match tokens.next() {
        Some(TokenTree::Ident(basis_name)) => {
            let (positive, negative, zero) = use_global_basis(|bs| {
                *bs.get(&basis_name.to_string())
                    .expect("Basis name was not registered, use define_basis!(...)")
            });
            symbolic_ga::basis::Basis {
                positive,
                negative,
                zero,
            }
        }
        _ => panic!("Basis name not specified in basis_types!(...)"),
    };

    let basis = macro_ga_logic::CodeBasis {
        scalar: "f32".to_string(),
        basis,
    };

    TokenStream::from(macro_ga_logic::structs::generate_types(&basis))
}

#[proc_macro]
pub fn ga(token_stream: TokenStream) -> TokenStream {
    function::function(token_stream)
}
