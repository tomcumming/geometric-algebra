use std::collections::BTreeMap;

use crate::basis::{Basis, SquaredElement};
use crate::element::Element;
use crate::symbols::Symbols;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiVector(BTreeMap<Element, Symbols>);

impl MultiVector {
    pub fn multiply(&self, basis: &Basis, rhs: &MultiVector) -> Result<MultiVector, String> {
        let mut result = MultiVector(BTreeMap::new());

        for (lhs_elem, lhs_sym) in self.0.iter() {
            for (rhs_elem, rhs_sym) in rhs.0.iter() {
                let sym = lhs_sym * rhs_sym;
                match lhs_elem.multiply(basis, rhs_elem)?.elems_and_sign() {
                    (SquaredElement::Zero, _) => {}
                    (SquaredElement::One, es) => {
                        let rhs = MultiVector(vec![(es, sym)].into_iter().collect());
                        result = result.add(rhs);
                    }
                    (SquaredElement::MinusOne, es) => {
                        let rhs = MultiVector(vec![(es, sym.invert())].into_iter().collect());
                        result = result.add(rhs);
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn add(self, rhs: MultiVector) -> MultiVector {
        self.0.into_iter().chain(rhs.0.into_iter()).fold(
            MultiVector(BTreeMap::new()),
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

    use crate::basis::Vector;
    use crate::symbols::lift_integer;

    use super::*;

    const G2: Basis = Basis {
        zero: 0,
        positive: 2,
        negative: 0,
    };

    #[test]
    fn test_simple_mult() {
        // (5 e2) (3a e2 + bb e1) = 15a - 5 bb e12
        // yeah the length of these expressions is absolutely bananas...
        let lhs = MultiVector(
            vec![(
                Element(vec![Vector(1)].into_iter().collect()),
                Symbols(
                    vec![(BTreeMap::new(), lift_integer(5))]
                        .into_iter()
                        .collect(),
                ),
            )]
            .into_iter()
            .collect(),
        );
        let rhs = MultiVector(
            vec![
                (
                    Element(vec![Vector(1)].into_iter().collect()),
                    Symbols(
                        vec![(
                            vec![("a".to_string(), 1)].into_iter().collect(),
                            lift_integer(3),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                ),
                (
                    Element(vec![Vector(0)].into_iter().collect()),
                    Symbols(
                        vec![(
                            vec![("b".to_string(), 2)].into_iter().collect(),
                            lift_integer(1),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let expected = MultiVector(
            vec![
                (
                    Element(BTreeSet::new()),
                    Symbols(
                        vec![(
                            vec![("a".to_string(), 1)].into_iter().collect(),
                            lift_integer(15),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                ),
                (
                    Element(vec![Vector(0), Vector(1)].into_iter().collect()),
                    Symbols(
                        vec![(
                            vec![("b".to_string(), 2)].into_iter().collect(),
                            lift_integer(-5),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(lhs.multiply(&G2, &rhs).unwrap(), expected);
    }
}
