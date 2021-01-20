use proc_macro::TokenStream;

#[proc_macro]
pub fn cool(_item: TokenStream) -> TokenStream {
    "123usize".parse().unwrap()
}
