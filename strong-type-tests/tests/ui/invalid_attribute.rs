use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(minimal_operators)]
struct Invalid(i32);

fn main() {}
