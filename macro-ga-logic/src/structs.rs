use std::str::FromStr;

use proc_macro2::TokenStream;

use crate::types::element_type_name;
use crate::CodeBasis;

pub fn generate_types(basis: &CodeBasis) -> TokenStream {
    let mut tokens = TokenStream::new();

    let elems = basis.basis.elements();

    for elem in elems {
        if elem.0.is_empty() {
            let scalar_def = TokenStream::from_str(&format!("type Scalar = {};", basis.scalar))
                .expect("Creating scalar definition");
            tokens.extend(scalar_def);
        } else {
            let src = format!(
                "#[derive(Debug, Copy, Clone, PartialEq)]\nstruct {}(pub {});",
                element_type_name(basis, &elem),
                basis.scalar
            );
            let struct_def = TokenStream::from_str(&src).expect("Creating struct definition");
            tokens.extend(struct_def);
        }
    }

    tokens
}
