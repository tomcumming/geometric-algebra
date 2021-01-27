use macro_ga;

macro_ga::define_basis!(PGA3, f32, 3, 0, 1);

macro_ga::basis_types!(PGA3);

fn main() {
    let f = macro_ga::ga! ( PGA3, |a: e1 + e2, b: 1| a * a + b );
    // After macro expansion
    // let f = |(E1(a_e1), E2(a_e2)): (E1, E2), b_1: f32| (a_e1 * a_e1 + a_e2 * a_e2 + b_1);

    let x = f((E1(1.0), E2(2.0)), 3.0);
    println!("Result {:?}", x);
}
