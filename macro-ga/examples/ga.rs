use macro_ga;

macro_ga::define_basis!(PGA2, 2, 0, 1);
macro_ga::define_basis!(PGA3, 3, 0, 1);

// type X = macro_ga::ga! ( PGA3, |a: 1 + e1 + e1e2e3| a );

fn main() {}
