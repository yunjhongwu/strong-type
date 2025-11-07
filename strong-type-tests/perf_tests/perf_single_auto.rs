// Performance benchmark: Single type with auto_operators
// This measures the baseline cost of a single fully-featured strong type

use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Price(i32);

fn main() {
    let p = Price::new(100);
    println!("{:?}", p);
}
