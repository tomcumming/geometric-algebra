use proc_macro2::{TokenStream, TokenTree};

pub fn tokenstream_push(tokens: &mut TokenStream, token: TokenTree) {
    tokens.extend(std::iter::once(token));
}
