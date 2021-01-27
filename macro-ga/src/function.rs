use proc_macro2::TokenTree;

use macro_ga_logic::parse::lambda::parse_lambda;

use crate::use_global_basis;

pub fn function(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream = proc_macro2::TokenStream::from(token_stream);

    let mut tokens = token_stream.into_iter().peekable();

    let basis = match tokens.next() {
        Some(TokenTree::Ident(basis_name)) => use_global_basis(|bs| {
            bs.get(&basis_name.to_string())
                .expect("Basis name was not registered, use define_basis!(...)")
                .clone()
        }),
        _ => panic!("Basis name not specified in ga!(...)"),
    };

    match tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {}
        token => panic!(
            "Expected ',' after basis name, got '{}'",
            token
                .map(|t| t.to_string())
                .unwrap_or_else(|| "EOS".to_string())
        ),
    };

    let pf =
        parse_lambda(&mut tokens).expect("There was a problem parsing the function inside ga!()");

    let tokens = pf
        .as_code(&basis)
        .expect("There was a problem generating code for function inside ga!()");

    proc_macro::TokenStream::from(tokens)
}
