// Performance benchmark: Five types with auto_operators
// Measures linear scaling of compilation time

use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Price(i32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Quantity(i32);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Distance(f64);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Weight(f64);

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Age(u32);

fn main() {
    let p = Price::new(100_i32);
    let q = Quantity::new(5_i32);
    let d = Distance::new(10.5_f64);
    let w = Weight::new(2.5_f64);
    let a = Age::new(25_u32);
    println!("{:?} {:?} {:?} {:?} {:?}", p, q, d, w, a);
}
