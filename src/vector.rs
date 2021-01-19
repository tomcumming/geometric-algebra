use std::marker::PhantomData;

use crate::basis::Basis;
use crate::element::SquaredElement;

#[derive(Debug)]
pub struct Vector<B: Basis>(PhantomData<B>, pub usize);

impl<B: Basis> Clone for Vector<B> {
    fn clone(&self) -> Self {
        match self {
            Vector(_, v) => Vector(PhantomData, *v),
        }
    }
}

impl<B: Basis> Copy for Vector<B> {}

impl<B: Basis> PartialEq for Vector<B> {
    fn eq(&self, rhs: &Self) -> bool {
        self.1 == rhs.1
    }
}

impl<B: Basis> Eq for Vector<B> {}

impl<B: Basis> PartialOrd for Vector<B> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&rhs.1)
    }
}

impl<B: Basis> Ord for Vector<B> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.1.cmp(&rhs.1)
    }
}

impl<B: Basis> Vector<B> {
    pub fn from_index(index: usize) -> Option<Vector<B>> {
        if index < B::ZERO + B::POSITIVE + B::NEGATIVE {
            Some(Vector(PhantomData, index))
        } else {
            None
        }
    }
}

impl<B: Basis> Vector<B> {
    pub fn square(&self) -> SquaredElement {
        let Vector(_, v) = self;

        if *v < B::ZERO {
            SquaredElement::Zero
        } else if *v < B::ZERO + B::POSITIVE {
            SquaredElement::One
        } else if *v < B::ZERO + B::POSITIVE + B::NEGATIVE {
            SquaredElement::MinusOne
        } else {
            unreachable!("Vector index OOB")
        }
    }
}
