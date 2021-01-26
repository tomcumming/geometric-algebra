use macro_ga;

macro_ga::define_basis!(PGA2, 2, 0, 1);
macro_ga::define_basis!(PGA3, 3, 0, 1);

#[derive(Debug)]
struct E1(f32);
#[derive(Debug)]
struct E2(f32);
#[derive(Debug)]
struct E1E2E3(f32);

fn main() {
    let f = macro_ga::ga! ( PGA3, |a: 1 + e1 + e1e2e3, b: 1| a + b );
    let x = f((1.0, E1(2.0), E1E2E3(3.0)), 4.0);
    println!("Result {:?}", x);
}
