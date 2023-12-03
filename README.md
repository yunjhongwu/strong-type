# named-type
Macros to create strong typed and named values in Rust.
 - `NamedType`: creates a strong type with a name.
 - `NamedNumeric`: creates a strong type with a name and implements traits for arithmetic operations.
 - `default_display`: implements `Display` trait for the type with the default format `TypeName(value)`.

## Supported underlying types:
 - Both `NamedType` and `NamedNumeric`:
   - `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
   - `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
   - `f32`, `f64`
 - Only `NamedType`:
   - `bool`
   - `char`
   - `String`

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
