use std::collections::BTreeMap;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro::TokenTree;

static mut GLOBAL_BASIS: Option<BTreeMap<String, (usize, usize, usize)>> = None;

fn use_global_basis<R, F: FnOnce(&mut BTreeMap<String, (usize, usize, usize)>) -> R>(f: F) -> R {
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
pub fn cool(_item: TokenStream) -> TokenStream {
    let found = use_global_basis(|basis| match basis.get("PGA2") {
        Some(b) => *b,
        None => panic!("no basis set"),
    });

    format!("{:?}", found).parse().unwrap()
}
