use std::collections::BTreeMap;

pub type Symbol = String;

pub type SymbolPowers = BTreeMap<Symbol, usize>;

#[derive(Clone)]
pub struct Symbols(pub BTreeMap<SymbolPowers, f32>);

impl Symbols {
    pub fn mult(self, Symbols(rhs_powers): &Symbols) -> Symbols {
        let Symbols(lhs_powers) = self;
        lhs_powers
            .iter()
            .flat_map(|(lhs_power, lhs_scale)| {
                rhs_powers.iter().map(move |(rhs_power, rhs_scale)| {
                    (mult_powers(lhs_power, &rhs_power), lhs_scale * rhs_scale)
                })
            })
            .fold(Symbols(BTreeMap::new()), |prev, (pwr, scale)| {
                prev.add_power(scale, pwr)
            })
    }

    fn add_power(mut self, scale: f32, power: SymbolPowers) -> Symbols {
        let existing = self.0.remove(&power).unwrap_or(0.0);
        let sum = existing + scale;
        if sum != 0.0 {
            self.0.insert(power, sum);
        }
        self
    }

    pub fn invert(self) -> Self {
        self.mult(&Symbols(
            vec![(BTreeMap::new(), -1.0)].into_iter().collect(),
        ))
    }

    pub fn add_syms(self, rhs: Self) -> Self {
        self.0
            .into_iter()
            .chain(rhs.0.into_iter())
            .fold(Symbols(BTreeMap::new()), |prev, (pows, scale)| {
                prev.add_power(scale, pows)
            })
    }
}

fn add_symbol_powers(lhs: SymbolPowers, rhs: SymbolPowers) -> SymbolPowers {
    let mut result: SymbolPowers = BTreeMap::new();

    for (sym, pow) in lhs.into_iter().chain(rhs.into_iter()) {
        let existing = *result.get(&sym).unwrap_or(&0);
        result.insert(sym, existing + pow);
    }

    result
}

fn mult_powers(lhs: &SymbolPowers, rhs: &SymbolPowers) -> SymbolPowers {
    lhs.iter()
        .flat_map(|(lhs_sym, lhs_pow)| {
            rhs.iter().map(move |(rhs_sym, rhs_pow)| {
                if lhs_sym == rhs_sym {
                    vec![(lhs_sym.to_string(), lhs_pow + rhs_pow)]
                        .into_iter()
                        .collect()
                } else {
                    vec![
                        (lhs_sym.to_string(), *lhs_pow),
                        (rhs_sym.to_string(), *rhs_pow),
                    ]
                    .into_iter()
                    .collect()
                }
            })
        })
        .fold(BTreeMap::new(), add_symbol_powers)
}
