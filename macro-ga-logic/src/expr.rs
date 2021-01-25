use std::collections::{BTreeMap, BTreeSet};

use symbolic_ga::element::Element;
use symbolic_ga::multivector::MultiVector;
use symbolic_ga::symbols::{lift_integer, Symbols};

use crate::{CodeBasis, Expr};

pub fn simplify_expr(_basis: &CodeBasis, expr: &Expr) -> Result<MultiVector, String> {
    match expr {
        Expr::Constant(x) => Ok(mv_from_scalar(*x)),
        _ => todo!(),
    }
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
