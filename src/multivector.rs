use std::collections::BTreeMap;

use crate::basis::Basis;
use crate::element::{Element, SimplifiedElement};
use crate::symbols::Symbols;

pub struct MultiVector<B: Basis>(BTreeMap<Element<B>, Symbols>);

impl<B: Basis> std::fmt::Debug for MultiVector<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("MultiVector")
            .field("elems", &self.0)
            .finish()
    }
}

impl<B: Basis> PartialEq for MultiVector<B> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl<B: Basis> MultiVector<B> {
    pub fn mult(self, rhs: &MultiVector<B>) -> MultiVector<B> {
        self.0
            .into_iter()
            .flat_map(|(lhs_elem, lhs_sym)| {
                rhs.0.iter().map(move |(rhs_elem, rhs_sym)| {
                    let sym = &lhs_sym * rhs_sym;
                    match &lhs_elem * rhs_elem {
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
                let sum = existing + sym;
                if !sum.0.is_empty() {
                    prev.0.insert(elem, sum);
                }
                prev
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use super::*;
    use crate::vector::Vector;

    struct G2;

    impl Basis for G2 {
        const ZERO: usize = 0;
        const POSITIVE: usize = 2;
        const NEGATIVE: usize = 0;
    }

    #[test]
    fn test_simple_mult() {
        // (5 e2) (3a e2 + bb e1) = 15a - 5 bb e12
        // yeah the length of these expressions is absolutely bananas...
        let lhs = MultiVector::<G2>(
            vec![(
                Element(
                    vec![Vector::<G2>::from_index(1).unwrap()]
                        .into_iter()
                        .collect(),
                ),
                Symbols(vec![(BTreeMap::new(), 5.0)].into_iter().collect()),
            )]
            .into_iter()
            .collect(),
        );
        let rhs = MultiVector::<G2>(
            vec![
                (
                    Element(
                        vec![Vector::<G2>::from_index(1).unwrap()]
                            .into_iter()
                            .collect(),
                    ),
                    Symbols(
                        vec![(vec![("a".to_string(), 1)].into_iter().collect(), 3.0)]
                            .into_iter()
                            .collect(),
                    ),
                ),
                (
                    Element(
                        vec![Vector::<G2>::from_index(0).unwrap()]
                            .into_iter()
                            .collect(),
                    ),
                    Symbols(
                        vec![(vec![("b".to_string(), 2)].into_iter().collect(), 1.0)]
                            .into_iter()
                            .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let expected = MultiVector::<G2>(
            vec![
                (
                    Element(BTreeSet::new()),
                    Symbols(
                        vec![(vec![("a".to_string(), 1)].into_iter().collect(), 15.0)]
                            .into_iter()
                            .collect(),
                    ),
                ),
                (
                    Element(
                        vec![
                            Vector::<G2>::from_index(0).unwrap(),
                            Vector::<G2>::from_index(1).unwrap(),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    Symbols(
                        vec![(vec![("b".to_string(), 2)].into_iter().collect(), -5.0)]
                            .into_iter()
                            .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(lhs.mult(&rhs), expected);
    }
}
