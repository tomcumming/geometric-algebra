use macro_ga;

macro_ga::define_basis!(PGA2, 2, 0, 1);
macro_ga::define_basis!(PGA3, 3, 0, 1);

struct E1(f32);
struct E2(f32);
struct E1E2E3(f32);

fn main() {
    let f = macro_ga::ga! ( PGA3, |a: 1 + e1 + e1e2e3, b: 1| a );
    let () = f((123.0, E1(1.0), E1E2E3(2.0)), 4.0);
    println!("that actually worked...")
}
