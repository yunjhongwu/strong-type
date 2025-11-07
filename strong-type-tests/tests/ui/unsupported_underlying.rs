use strong_type::StrongType;

#[derive(StrongType)]
struct Base(i32);

#[derive(StrongType)]
#[strong_type(underlying=not::a::primitive)]
struct Unsupported(Base);

fn main() {}
