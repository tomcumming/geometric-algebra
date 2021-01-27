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

pub fn all_vectors(basis: &Basis) -> Vec<Vector> {
    (0..basis.zero + basis.positive + basis.negative)
        .map(Vector)
        .collect()
}
