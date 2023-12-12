# strong-type
`strong-type` is a Rust crate that offers macros to easily create strongly typed and named primitive and string types. Strong typing helps in making code more expressive and less prone to errors, ensuring that each type is used in its intended way.

```rust
use strong_type::StrongType;

#[derive(StrongType)]
struct Timestamp(i64);

let timestamp = Timestamp::new(1701620628123456789);
println!("{}", timestamp); // Timestamp(1701620628123456789)
```

## Features
- `StrongType`: Create a named strong type. The macro automatically implement common traits like `Clone`, `Debug`, `Default`, `PartialEq`, `PartialOrd`, `Send`, `Sync`, and `Display` (unless the `custom_display` attribute is used to override it). Additionally, depending on the underlying data type, strong-typed structs may also implement `Copy`, `Eq`, `Ord`, `Hash`. For example, if the underlying type is a primitive data type like `i32` or `bool`, these additional traits will be implemented. This allows the strong types to inherit useful behaviors from their underlying types, while still maintaining their distinct identity at the type level.
- `StrongNumericType`: Extend `StrongType` with arithmetic/logical operations.
- `custom_display`: Provides flexibility for users to manually implement `Display` instead of using the default display format.

## Installation
Add `strong-type` to your `Cargo.toml`:
```toml
[dependencies]
strong-type = "*" # Using the latest version or specify a version number
```

## Supported underlying types:
 - Both `StrongType` and `StrongNumericType`:
   - Integer types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
   - Unsigned integer types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
   - Floating-point types: `f32`, `f64`
   - Boolean type: `bool`
 - Only `StrongType`:
   - `char`
   - `String`

## Examples
#### Creating a named strong type:
```rust
use strong_type::StrongType;

#[derive(StrongType)]
struct Tag(String);

let tag = Tag::new("dev");
let tag: Tag = "dev".into(); // Alternative instantiation
```

#### Demonstrating type distinctiveness:

```rust
use strong_type::StrongType;
use std::any::Any;

#[derive(StrongType)]
struct Second(i32);

#[derive(StrongType)]
struct Minute(i32);

let x = Second::new(2);
let y = Second::new(3);
let z = Minute::new(3);

assert_eq!(x.type_id(), y.type_id()); // Same type: Second
assert_ne!(y.type_id(), z.type_id()); // Different types: Second versus Minute
```

#### Utilizing Hashability:

```rust
use std::collections::HashSet;

#[derive(StrongType)]
struct Tag(String);

let mut map = HashSet::<Tag>::new();
map.insert(Tag::new("dev"));
map.insert(Tag::new("prod"));
assert_eq!(map.len(), 2);
```

#### Named integer type with arithmetic operations:

```rust
use strong_type::StrongNumericType;

#[derive(StrongNumericType)]
struct Second(i32);

let x = Second::new(2);
let y = Second::new(3);
let z = Second::default();

assert_eq!(x.value(), 2);
assert_eq!(y.value(), 3);
assert_eq!(z.value(), 0);
assert!(x < y);
assert!(y >= x);
assert_eq!(x + y, Second(5));
```

#### Named bool type with logical operations:

```rust
use strong_type::StrongNumericType;

#[derive(StrongNumericType)]
struct IsTrue(bool);

let x = IsTrue::new(true);
let y = IsTrue::new(false);

assert_eq!(x & y, IsTrue::new(false));
assert_eq!(x | y, IsTrue::new(true));
assert_eq!(x ^ y, IsTrue::new(true));
assert_eq!(!x, IsTrue::new(false));
```

#### Custom display implementation with `custom_display`:

```rust
use std::fmt::{Display, Formatter, Result};
use strong_type::StrongNumericType;

#[derive(StrongNumericType)]
#[custom_display]
struct Second(f64);

impl Display for Second {
   fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "Second({:.2})", &self.0)
   }
}

println!("{}", Second::new(std::f64::consts::E)); // "Second(2.72)"
println!("{:?}", Second::new(std::f64::consts::E)); // "Second { value: 2.718281828459045 }"
```
