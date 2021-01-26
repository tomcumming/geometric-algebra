use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;

use num::rational::BigRational;
use num::One;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use symbolic_ga::element::Element;
use symbolic_ga::multivector::MultiVector;
use symbolic_ga::symbols::{lift_integer, SymbolPowers, Symbols};

use crate::tokens::tokenstream_push;
use crate::types::{element_term_name, element_type_name};
use crate::{CodeBasis, Expr, MVType};

pub fn simplify_expr(
    basis: &CodeBasis,
    sym_types: &BTreeMap<String, MVType>,
    expr: &Expr,
) -> Result<MultiVector, String> {
    match expr {
        Expr::Symbol(sym) => Ok(symbol_as_mv(sym_types, sym)),
        Expr::Constant(x) => Ok(mv_from_scalar(*x)),
        Expr::Add(a, b) => {
            let mv_a = simplify_expr(basis, sym_types, a)?;
            let mv_b = simplify_expr(basis, sym_types, b)?;
            Ok(mv_a + mv_b)
        }
        Expr::Mul(a, b) => {
            let mv_a = simplify_expr(basis, sym_types, a)?;
            let mv_b = simplify_expr(basis, sym_types, b)?;
            mv_a.multiply(&basis.basis, &mv_b)
        }
        _ => todo!("simplify expr"),
    }
}

pub fn mv_as_code(basis: &CodeBasis, mv: &MultiVector) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (elem, syms) in mv.0.iter() {
        if !tokens.is_empty() {
            tokenstream_push(&mut tokens, Punct::new(',', Spacing::Alone).into());
        }

        if elem.0.is_empty() {
            tokens.extend(symbols_as_code(syms))
        } else {
            let type_name = element_type_name(basis, elem);
            tokenstream_push(
                &mut tokens,
                Ident::new(&type_name, Span::call_site()).into(),
            );
            tokenstream_push(
                &mut tokens,
                Group::new(Delimiter::Parenthesis, symbols_as_code(syms)).into(),
            );
        }
    }

    std::iter::once::<TokenTree>(Group::new(Delimiter::Parenthesis, tokens).into()).collect()
}

fn symbol_as_mv(sym_types: &BTreeMap<String, MVType>, sym: &str) -> MultiVector {
    // Should have been checked before code gen
    let types = sym_types.get(sym).expect("Symbol missing in context");

    let mut mv = MultiVector::default();

    for elem in types.0.iter() {
        mv = mv + mv_from_symbol(format!("{}_{}", sym, element_term_name(elem)), elem.clone());
    }

    mv
}

fn symbols_as_code(syms: &Symbols) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (powers, scale) in syms.0.iter() {
        if !tokens.is_empty() {
            tokenstream_push(&mut tokens, Punct::new('+', Spacing::Alone).into());
        }

        if !scale.is_one() || powers.is_empty() {
            tokens.extend(rational_as_code(scale));

            if !powers.is_empty() {
                tokenstream_push(&mut tokens, Punct::new('*', Spacing::Alone).into());
            }
        }

        tokens.extend(powers_as_code(powers));
    }

    tokens
}

fn rational_as_code(rat: &BigRational) -> TokenStream {
    let numer = usize::try_from(rat.numer()).expect("Could not convert numer from big rational");
    if rat.denom().is_one() {
        std::iter::once::<TokenTree>(Literal::f64_unsuffixed(numer as f64).into()).collect()
    } else {
        todo!("rational as code")
    }
}

fn powers_as_code(powers: &SymbolPowers) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (sym, pow) in powers.iter() {
        for _ in 0..*pow {
            if !tokens.is_empty() {
                tokenstream_push(&mut tokens, Punct::new('*', Spacing::Alone).into());
            }
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

fn mv_from_symbol(x: String, elem: Element) -> MultiVector {
    MultiVector(
        vec![(
            elem,
            Symbols(
                vec![(vec![(x, 1)].into_iter().collect(), lift_integer(1))]
                    .into_iter()
                    .collect(),
            ),
        )]
        .into_iter()
        .collect(),
    )
}
