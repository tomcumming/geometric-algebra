use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::basis::Basis;
use crate::vector::Vector;

pub enum SquaredElement {
    Zero,
    One,
    MinusOne,
}

pub struct Element<B: Basis>(pub BTreeSet<Vector<B>>);

impl<B: Basis> std::fmt::Debug for Element<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Element").field("elems", &self.0).finish()
    }
}

impl<B: Basis> PartialEq for Element<B> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl<B: Basis> Eq for Element<B> {}

impl<B: Basis> PartialOrd for Element<B> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl<B: Basis> Ord for Element<B> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.0.cmp(&rhs.0)
    }
}

pub enum SimplifiedElement<B: Basis> {
    Zero,
    Positive(Element<B>),
    Negative(Element<B>),
}

impl<B: Basis> std::fmt::Debug for SimplifiedElement<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SimplifiedElement::Zero => 0.fmt(f),
            SimplifiedElement::Positive(es) => f.write_fmt(format_args!("+{:?}", es)),
            SimplifiedElement::Negative(es) => f.write_fmt(format_args!("-{:?}", es)),
        }
    }
}

impl<B: Basis> PartialEq for SimplifiedElement<B> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (SimplifiedElement::Zero, SimplifiedElement::Zero) => true,
            (SimplifiedElement::Positive(es), SimplifiedElement::Positive(es2)) => es == es2,
            (SimplifiedElement::Negative(es), SimplifiedElement::Negative(es2)) => es == es2,
            _ => false,
        }
    }
}

impl<B: Basis> SimplifiedElement<B> {
    pub fn elems(self) -> Option<Element<B>> {
        match self {
            SimplifiedElement::Zero => None,
            SimplifiedElement::Positive(es) => Some(es),
            SimplifiedElement::Negative(es) => Some(es),
        }
    }

    pub fn flip(self) -> SimplifiedElement<B> {
        match self {
            SimplifiedElement::Zero => SimplifiedElement::Zero,
            SimplifiedElement::Positive(es) => SimplifiedElement::Negative(es),
            SimplifiedElement::Negative(es) => SimplifiedElement::Positive(es),
        }
    }

    pub fn then<F: FnOnce(Element<B>) -> SimplifiedElement<B>>(self, f: F) -> Self {
        match self {
            SimplifiedElement::Zero => SimplifiedElement::Zero,
            SimplifiedElement::Positive(es) => f(es),
            SimplifiedElement::Negative(es) => f(es).flip(),
        }
    }
}

impl<B: Basis> Element<B> {
    fn mul_left(self, left: Vector<B>) -> SimplifiedElement<B> {
        let Element(mut vs) = self;
        match vs.pop_first() {
            None => SimplifiedElement::Positive(Element(vec![left].into_iter().collect())),
            Some(first_v) => match first_v.cmp(&left) {
                Ordering::Greater => {
                    vs.insert(first_v);
                    vs.insert(left);
                    SimplifiedElement::Positive(Element(vs))
                }
                Ordering::Equal => match left.square() {
                    SquaredElement::Zero => SimplifiedElement::Zero,
                    SquaredElement::One => SimplifiedElement::Positive(Element(vs)),
                    SquaredElement::MinusOne => SimplifiedElement::Negative(Element(vs)),
                },
                Ordering::Less => Element(vs).mul_left(left).then(|mut es| {
                    es.0.insert(first_v);
                    SimplifiedElement::Negative(es) // causes a flip
                }),
            },
        }
    }

    pub fn mult(&self, rhs: &Element<B>) -> SimplifiedElement<B> {
        let mut elems: Vec<Vector<B>> = self.0.iter().cloned().collect();
        elems.reverse();

        elems.into_iter().fold(
            SimplifiedElement::Positive(Element(rhs.0.clone())),
            |prev, curr| prev.then(|es| es.mul_left(curr)),
        )
    }
}

impl<B: Basis> From<Vector<B>> for Element<B> {
    fn from(v: Vector<B>) -> Element<B> {
        Element(vec![v].into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OneTwoOne {}

    impl Basis for OneTwoOne {
        const ZERO: usize = 1;
        const POSITIVE: usize = 2;
        const NEGATIVE: usize = 1;
    }

    #[test]
    fn test_bivector_squares_to_minus_one() {
        let e1 = Vector::<OneTwoOne>::from_index(1).unwrap();
        let e2 = Vector::<OneTwoOne>::from_index(2).unwrap();

        let e2: Element<OneTwoOne> = e2.into();
        let e12 = e2.mul_left(e1);

        match e12 {
            SimplifiedElement::Positive(e12) => {
                assert_eq!(
                    e12.mult(&e12),
                    SimplifiedElement::Negative(Element(BTreeSet::new()))
                );
            }
            _ => panic!("Could not construct bivector"),
        }
    }

    #[test]
    fn test_squares_to_zero() {
        let e0 = Vector::<OneTwoOne>::from_index(0).unwrap();
        let e1 = Vector::<OneTwoOne>::from_index(1).unwrap();
        let e2 = Vector::<OneTwoOne>::from_index(2).unwrap();

        let e1: Element<OneTwoOne> = e1.into();
        let e2: Element<OneTwoOne> = e2.into();

        let e01 = e1.mul_left(e0);
        let e02 = e2.mul_left(e0);

        match (e01, e02) {
            (SimplifiedElement::Positive(e01), SimplifiedElement::Positive(e02)) => {
                assert_eq!(e01.mult(&e02), SimplifiedElement::Zero,);
            }
            _ => panic!("Could not construct bivectors"),
        }
    }
}
