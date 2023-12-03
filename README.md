# named-type
`named-type` is a Rust crate providing macros to create strongly typed and named values. It simplifies the process of defining types that are distinct at the type level but share underlying data structures. 
```rust
use named_type::NamedType;

#[derive(NamedType)]
struct Timestamp(i64);

let timestamp = Timestamp(1701620628123456789);
println!("{}", timestamp); // Timestamp(1701620628123456789)
```

## Features
- `NamedType`: Create a named strong type.
- `NamedNumeric`: Extend `NamedType` with arithmetic operations.
- `custom_display`: Allow users to manually implement `Display` instead of using the default display format.

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
#### Create a named type:
```rust
use named_type::NamedType;

#[derive(NamedType)]
struct Tag(String);

let _ = Tag("dev".to_string());
let tag = Tag::new("dev");
let tag: Tag = "dev".into();
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

assert_eq!(x.type_id(), y.type_id()); // Same type: Second
assert_ne!(y.type_id(), z.type_id()); // Different type: Second versus Minute
```

#### Named type with arithmetic operations:

```rust
use named_type::NamedNumeric;

#[derive(NamedNumeric)]
struct Second(i32);

let x = Second(2);
let y = Second(3);

assert_eq!(x.value(), 2);
assert_eq!(y.value(), 3);
assert!(x < y);
assert!(y >= x);
assert_eq!(x + y, Second(5));
```

#### Named type with `custom_display`:

```rust
use named_type::NamedNumeric;

#[derive(NamedNumeric)]
#[custom_display]
struct Second(f64);

impl Display for Second {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Mile({:.2})", &self.0)
   }
}

println!("{}", Second(std::f64::consts::E)); // "Second(2.72)"
println!("{:?}", Second(std::f64::consts::E)); // "Second { value: 2.718281828459045 }"
```
