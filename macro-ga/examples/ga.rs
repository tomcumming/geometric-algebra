use macro_ga;

macro_ga::define_basis!(PGA2, 2, 0, 1);
macro_ga::define_basis!(PGA3, 3, 0, 1);

fn main() {
    println!("Hello World! {:?}", macro_ga::cool!());
}
