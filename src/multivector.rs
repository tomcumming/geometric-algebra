use std::collections::BTreeMap;

use crate::basis::Basis;
use crate::element::{Element, SimplifiedElement};
use crate::symbols::Symbols;

pub struct MultiVector<B: Basis>(BTreeMap<Element<B>, Symbols>);

impl<B: Basis> MultiVector<B> {
    pub fn mult(self, rhs: &MultiVector<B>) -> MultiVector<B> {
        self.0
            .into_iter()
            .flat_map(|(lhs_elem, lhs_sym)| {
                rhs.0.iter().map(move |(rhs_elem, rhs_sym)| {
                    let sym = lhs_sym.clone().mult(rhs_sym);
                    match lhs_elem.mult(rhs_elem) {
                        SimplifiedElement::Zero => MultiVector::<B>(BTreeMap::new()),
                        SimplifiedElement::Positive(elem) => {
                            MultiVector(vec![(elem, sym)].into_iter().collect())
                        }
                        SimplifiedElement::Negative(elem) => {
                            MultiVector(vec![(elem, sym.invert())].into_iter().collect())
                        }
                    }
                })
            })
            .fold(MultiVector::<B>(BTreeMap::new()), |prev, curr| {
                prev.add_mv(curr)
            })
    }

    pub fn add_mv(self, rhs: MultiVector<B>) -> MultiVector<B> {
        self.0.into_iter().chain(rhs.0.into_iter()).fold(
            MultiVector::<B>(BTreeMap::new()),
            |mut prev, (elem, sym)| {
                let existing = prev
                    .0
                    .remove(&elem)
                    .unwrap_or_else(|| Symbols(BTreeMap::new()));
                let sum = existing.add_syms(sym);
                if !sum.0.is_empty() {
                    prev.0.insert(elem, sum);
                }
                prev
            },
        )
    }
}
