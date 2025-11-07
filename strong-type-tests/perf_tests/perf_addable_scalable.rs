// Performance benchmark: Types with selective operators
// Measures impact of using only addable and scalable instead of full auto_operators

use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(addable, scalable)]
struct Price(i32);

#[derive(StrongType)]
#[strong_type(addable)]
struct Quantity(i32);

#[derive(StrongType)]
#[strong_type(scalable)]
struct Weight(f64);

#[derive(StrongType)]
struct SimpleId(u64);

fn main() {
    let p = Price::new(100_i32);
    let q = Quantity::new(5_i32);
    let w = Weight::new(2.5_f64);
    let id = SimpleId::new(12345_u64);
    println!("{:?} {:?} {:?} {:?}", p, q, w, id);
}
