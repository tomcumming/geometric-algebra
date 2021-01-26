use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::basis::{Basis, SquaredElement, Vector};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Element(pub BTreeSet<Vector>);

#[derive(Debug, Clone, PartialEq)]
pub enum SimplifiedElement {
    Zero,
    Positive(Element),
    Negative(Element),
}

fn pop_first_vector(vs: &mut BTreeSet<Vector>) -> Option<Vector> {
    let v = vs.iter().cloned().next();
    v.and_then(|v| vs.take(&v))
}

impl SimplifiedElement {
    pub fn elems_and_sign(self) -> (SquaredElement, Element) {
        match self {
            SimplifiedElement::Zero => (SquaredElement::Zero, Element(BTreeSet::new())),
            SimplifiedElement::Positive(es) => (SquaredElement::One, es),
            SimplifiedElement::Negative(es) => (SquaredElement::MinusOne, es),
        }
    }

    pub fn flip(self) -> SimplifiedElement {
        match self {
            SimplifiedElement::Zero => SimplifiedElement::Zero,
            SimplifiedElement::Positive(es) => SimplifiedElement::Negative(es),
            SimplifiedElement::Negative(es) => SimplifiedElement::Positive(es),
        }
    }

    pub fn map(self, f: impl FnOnce(Element) -> Element) -> SimplifiedElement {
        match self {
            SimplifiedElement::Zero => SimplifiedElement::Zero,
            SimplifiedElement::Positive(es) => SimplifiedElement::Positive(f(es)),
            SimplifiedElement::Negative(es) => SimplifiedElement::Negative(f(es)),
        }
    }
}

impl Element {
    fn multiply_vector_left(
        self,
        basis: &Basis,
        left: Vector,
    ) -> Result<SimplifiedElement, String> {
        let Element(mut vs) = self;
        match pop_first_vector(&mut vs) {
            None => Ok(SimplifiedElement::Positive(Element(
                vec![left].into_iter().collect(),
            ))),
            Some(first_v) => match first_v.cmp(&left) {
                Ordering::Greater => {
                    vs.insert(first_v);
                    vs.insert(left);
                    Ok(SimplifiedElement::Positive(Element(vs)))
                }
                Ordering::Equal => match left.square(basis)? {
                    SquaredElement::Zero => Ok(SimplifiedElement::Zero),
                    SquaredElement::One => Ok(SimplifiedElement::Positive(Element(vs))),
                    SquaredElement::MinusOne => Ok(SimplifiedElement::Negative(Element(vs))),
                },
                Ordering::Less => {
                    let rest = Element(vs).multiply_vector_left(basis, left)?;
                    let rest = rest.map(|mut element| {
                        element.0.insert(first_v);
                        element
                    });
                    Ok(rest.flip())
                }
            },
        }
    }

    pub fn multiply(&self, basis: &Basis, rhs: &Element) -> Result<SimplifiedElement, String> {
        let mut elems: Vec<Vector> = self.0.iter().cloned().collect();
        elems.reverse();

        let mut curr = SimplifiedElement::Positive(Element(rhs.0.clone()));

        for lhs_rhs in elems {
            match curr.elems_and_sign() {
                (SquaredElement::Zero, _) => return Ok(SimplifiedElement::Zero),
                (sign, elems) => match elems.multiply_vector_left(basis, lhs_rhs)? {
                    SimplifiedElement::Zero => return Ok(SimplifiedElement::Zero),
                    SimplifiedElement::Positive(es) => {
                        curr = if sign == SquaredElement::MinusOne {
                            SimplifiedElement::Negative(es)
                        } else {
                            SimplifiedElement::Positive(es)
                        }
                    }
                    SimplifiedElement::Negative(es) => {
                        curr = if sign == SquaredElement::MinusOne {
                            SimplifiedElement::Positive(es)
                        } else {
                            SimplifiedElement::Negative(es)
                        }
                    }
                },
            }
        }

        Ok(curr)
    }
}

impl From<Vector> for Element {
    fn from(v: Vector) -> Element {
        Element(vec![v].into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONETWOONE: Basis = Basis {
        zero: 1,
        positive: 2,
        negative: 1,
    };

    #[test]
    fn test_bivector_squares_to_minus_one() -> Result<(), String> {
        let e1 = Vector(1);
        let e2: Element = Vector(2).into();

        let e12 = e2.multiply_vector_left(&ONETWOONE, e1)?;

        match e12 {
            SimplifiedElement::Positive(e12) => {
                assert_eq!(
                    e12.multiply(&ONETWOONE, &e12)?,
                    SimplifiedElement::Negative(Element(BTreeSet::new()))
                );
                Ok(())
            }
            _ => Err("Could not construct bivector".to_string()),
        }
    }

    #[test]
    fn test_squares_to_zero() -> Result<(), String> {
        let e0 = Vector(0);
        let e1: Element = Vector(1).into();
        let e2: Element = Vector(2).into();

        let e01 = e1.multiply_vector_left(&ONETWOONE, e0)?;
        let e02 = e2.multiply_vector_left(&ONETWOONE, e0)?;

        match (e01, e02) {
            (SimplifiedElement::Positive(e01), SimplifiedElement::Positive(e02)) => {
                assert_eq!(e01.multiply(&ONETWOONE, &e02)?, SimplifiedElement::Zero,);
                Ok(())
            }
            _ => Err("Could not construct bivectors".to_string()),
        }
    }
}
