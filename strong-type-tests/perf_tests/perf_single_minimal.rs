// Performance benchmark: Single type with auto_operators = "minimal"
// Compares against auto_operators = "full" to measure the impact of reduced trait generation

use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators = "minimal")]
struct Price(i32);

fn main() {
    let p = Price::new(100);
    println!("{:?}", p);
}
