# named-type
Macros to create strong typed and named values in Rust.

## Examples
```rust
use named_type::NamedType;

#[derive(NamedType)]
struct Tag(String);

let tag = Tag("dev".to_string());
println!("{:?}", tag); // Tag { value: "dev" }
```

#### Strong type:

```rust
use named_type::NamedType;
use std::any::Any;

#[derive(NamedType)]
struct Second(i32);

#[derive(NamedType)]
struct Minute(i32);

let x = Second(2);
let y = Second(3);
let z = Minute(3);

assert_eq!(x.type_id(), y.type_id());
assert_ne!(y.type_id(), z.type_id());
```

#### Named type with arithmetic operations:

```rust
use named_type::NamedNumeric;

#[derive(NamedNumeric)]
struct Second(i32);

let x = Second(2);
assert_eq!(x.value(), 2);
let mut y = Second(3);
assert!(x < y);
assert!(y >= x);
```

#### Named type with `default_display`:

```rust
use named_type::NamedNumeric;

#[derive(NamedNumeric)]
#[default_display]
struct Second(i32);

println!("{}", Second(2)); // "Second(2)"
println!("{:?}", Second(2)); // "Second { value: 2 }"
```
