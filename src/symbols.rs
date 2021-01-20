use std::collections::BTreeMap;

pub type Symbol = String;

pub type SymbolPowers = BTreeMap<Symbol, usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbols(pub BTreeMap<SymbolPowers, f32>);

impl<'a> std::ops::Mul for &'a Symbols {
    type Output = Symbols;

    fn mul(self, Symbols(rhs_powers): &Symbols) -> Symbols {
        let Symbols(lhs_powers) = self;
        lhs_powers
            .iter()
            .flat_map(|(lhs_power, lhs_scale)| {
                rhs_powers.iter().map(move |(rhs_power, rhs_scale)| {
                    (
                        multiply_symbol_powers(lhs_power, &rhs_power),
                        lhs_scale * rhs_scale,
                    )
                })
            })
            .fold(Symbols(BTreeMap::new()), |prev, (pwr, scale)| {
                prev.add_scaled_power(scale, pwr)
            })
    }
}

impl std::ops::Add for Symbols {
    type Output = Symbols;

    fn add(self, rhs: Symbols) -> Symbols {
        self.0
            .into_iter()
            .chain(rhs.0.into_iter())
            .fold(Symbols(BTreeMap::new()), |prev, (pows, scale)| {
                prev.add_scaled_power(scale, pows)
            })
    }
}

impl Symbols {
    fn add_scaled_power(mut self, scale: f32, power: SymbolPowers) -> Symbols {
        let existing = self.0.remove(&power).unwrap_or(0.0);
        let sum = existing + scale;
        if sum != 0.0 {
            self.0.insert(power, sum);
        }
        self
    }

    pub fn invert(&self) -> Self {
        self * &Symbols(vec![(BTreeMap::new(), -1.0)].into_iter().collect())
    }
}

fn multiply_symbol_powers(lhs: &SymbolPowers, rhs: &SymbolPowers) -> SymbolPowers {
    lhs.iter()
        .chain(rhs.iter())
        .fold(BTreeMap::new(), |mut prev, (sym, pow)| {
            let existing = prev.remove(sym).unwrap_or(0);
            prev.insert(sym.to_string(), existing + pow);
            prev
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mult_symbol_powers() {
        let lhs: SymbolPowers = vec![("x".to_string(), 2), ("y".to_string(), 3)]
            .into_iter()
            .collect();
        let rhs: SymbolPowers = vec![("y".to_string(), 4), ("z".to_string(), 5)]
            .into_iter()
            .collect();

        assert_eq!(
            mult_symbol_powers(&lhs, &rhs),
            vec![
                ("x".to_string(), 2),
                ("y".to_string(), 3 + 4),
                ("z".to_string(), 5)
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_mult_power_increase() {
        // (2 + x) * (3 + xx) = 6 + 2xx + 3x + xxx
        let lhs = Symbols(
            vec![
                (vec![("x".to_string(), 1)].into_iter().collect(), 1.0),
                (BTreeMap::new(), 2.0),
            ]
            .into_iter()
            .collect(),
        );
        let rhs = Symbols(
            vec![
                (vec![("x".to_string(), 2)].into_iter().collect(), 1.0),
                (BTreeMap::new(), 3.0),
            ]
            .into_iter()
            .collect(),
        );

        let expected = Symbols(
            vec![
                (vec![("x".to_string(), 3)].into_iter().collect(), 1.0),
                (vec![("x".to_string(), 2)].into_iter().collect(), 2.0),
                (vec![("x".to_string(), 1)].into_iter().collect(), 3.0),
                (BTreeMap::new(), 6.0),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(lhs.mult(&rhs), expected);
    }
}
