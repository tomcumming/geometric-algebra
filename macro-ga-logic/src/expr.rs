use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;

use num::rational::BigRational;
use num::One;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use symbolic_ga::element::Element;
use symbolic_ga::multivector::MultiVector;
use symbolic_ga::symbols::{lift_integer, SymbolPowers, Symbols};

use crate::tokens::tokenstream_push;
use crate::{CodeBasis, Expr};

pub fn simplify_expr(basis: &CodeBasis, expr: &Expr) -> Result<MultiVector, String> {
    match expr {
        Expr::Constant(x) => Ok(mv_from_scalar(*x)),
        Expr::Add(a, b) => {
            let mv_a = simplify_expr(basis, a)?;
            let mv_b = simplify_expr(basis, b)?;
            Ok(mv_a + mv_b)
        }
        _ => todo!(),
    }
}

pub fn mv_as_code(mv: &MultiVector) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (elem, syms) in mv.0.iter() {
        if !tokens.is_empty() {
            tokenstream_push(&mut tokens, Punct::new(',', Spacing::Alone).into());
        }

        if elem.0.is_empty() {
            tokens.extend(symbols_as_code(syms))
        } else {
            todo!()
        }
    }

    std::iter::once::<TokenTree>(Group::new(Delimiter::Parenthesis, tokens).into()).collect()
}

fn symbols_as_code(syms: &Symbols) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (powers, scale) in syms.0.iter() {
        if !tokens.is_empty() {
            tokenstream_push(&mut tokens, Punct::new('+', Spacing::Alone).into());
        }

        tokens.extend(rational_as_code(scale));

        powers_as_code(powers);
    }

    tokens
}

fn rational_as_code(rat: &BigRational) -> TokenStream {
    let numer = usize::try_from(rat.numer()).expect("Could not convert numer from big rational");
    if rat.denom().is_one() {
        std::iter::once::<TokenTree>(Literal::usize_unsuffixed(numer).into()).collect()
    } else {
        todo!()
    }
}

fn powers_as_code(powers: &SymbolPowers) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (sym, pow) in powers.iter() {
        for _ in 0..*pow {
            tokenstream_push(&mut tokens, Punct::new('*', Spacing::Alone).into());
            tokenstream_push(&mut tokens, Ident::new(sym, Span::call_site()).into());
        }
    }

    tokens
}

fn mv_from_scalar(x: isize) -> MultiVector {
    MultiVector(
        vec![(
            Element(BTreeSet::new()),
            Symbols(
                vec![(BTreeMap::new(), lift_integer(x))]
                    .into_iter()
                    .collect(),
            ),
        )]
        .into_iter()
        .collect(),
    )
}
