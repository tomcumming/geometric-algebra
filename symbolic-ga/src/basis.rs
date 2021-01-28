use std::collections::BTreeSet;

use crate::element::Element;

#[derive(Debug, Clone)]
pub struct Basis {
    pub zero: usize,
    pub positive: usize,
    pub negative: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquaredElement {
    Zero,
    One,
    MinusOne,
}

pub type Grade = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector(pub usize);

impl Vector {
    pub fn square(&self, basis: &Basis) -> Result<SquaredElement, String> {
        match self.0 {
            idx if idx < basis.zero => Ok(SquaredElement::Zero),
            idx if idx < basis.zero + basis.positive => Ok(SquaredElement::One),
            idx if idx < basis.zero + basis.positive + basis.negative => {
                Ok(SquaredElement::MinusOne)
            }
            idx => Err(format!("Vector index is larger than basis: {}", idx)),
        }
    }
}

impl Basis {
    pub fn vectors(&self) -> Vec<Vector> {
        (0..self.zero + self.positive + self.negative)
            .map(Vector)
            .collect()
    }

    pub fn grade(&self, n: Grade) -> Vec<Element> {
        combinations(&self.vectors(), n)
            .into_iter()
            .map(Element)
            .collect()
    }

    pub fn elements(&self) -> Vec<Element> {
        (0..=self.zero + self.positive + self.negative)
            .flat_map(|grade| self.grade(grade))
            .collect()
    }
}

fn combinations<T: Clone + Ord>(xs: &[T], n: usize) -> Vec<BTreeSet<T>> {
    match n {
        _ if n > xs.len() => Vec::new(),
        0 => vec![BTreeSet::new()],
        n => combinations(&xs[1..], n - 1)
            .into_iter()
            .map(|mut ys| {
                ys.insert(xs[0].clone());
                ys
            })
            .chain(combinations(&xs[1..], n))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const G3: Basis = Basis {
        zero: 0,
        positive: 3,
        negative: 0,
    };

    #[test]
    fn test_grade_zero() {
        assert_eq!(G3.grade(0), vec![Element(BTreeSet::new())]);
    }

    #[test]
    fn test_grade_one() {
        assert_eq!(
            G3.grade(1),
            vec![0, 1, 2]
                .into_iter()
                .map(|v| Element(vec![Vector(v)].into_iter().collect()))
                .collect::<Vec<Element>>()
        );
    }

    #[test]
    fn test_grade_two() {
        assert_eq!(
            G3.grade(2),
            vec![vec![0, 1], vec![0, 2], vec![1, 2]]
                .into_iter()
                .map(|vs| Element(vs.into_iter().map(Vector).collect()))
                .collect::<Vec<Element>>()
        );
    }

    #[test]
    fn test_grade_three() {
        assert_eq!(
            G3.grade(3),
            vec![vec![0, 1, 2]]
                .into_iter()
                .map(|vs| Element(vs.into_iter().map(Vector).collect()))
                .collect::<Vec<Element>>()
        );
    }

    #[test]
    fn test_all_elements_for_g3() {
        let expected: Vec<Element> = vec![
            Element(BTreeSet::new()),
            Element(vec![Vector(0)].into_iter().collect()),
            Element(vec![Vector(1)].into_iter().collect()),
            Element(vec![Vector(2)].into_iter().collect()),
            Element(vec![Vector(0), Vector(1)].into_iter().collect()),
            Element(vec![Vector(0), Vector(2)].into_iter().collect()),
            Element(vec![Vector(1), Vector(2)].into_iter().collect()),
            Element(vec![Vector(0), Vector(1), Vector(2)].into_iter().collect()),
        ];

        assert_eq!(G3.elements(), expected);
    }
}
