use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::basis::Basis;
use crate::vector::Vector;

pub enum SquaredElement {
    Zero,
    One,
    MinusOne,
}

pub struct Element<B: Basis>(BTreeSet<Vector<B>>);

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
                    vs.insert(left);
                    SimplifiedElement::Positive(Element(vs))
                }
                Ordering::Equal => match left.square() {
                    SquaredElement::Zero => SimplifiedElement::Positive(Element(vs)),
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
