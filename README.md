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

- **Derive trait `StrongType`:** Create a named strong type. 
  - The macro automatically implement common traits like `Clone`, `Debug`, `Default`, `PartialEq`, `PartialOrd`, `Send`, and `Sync`. It also implements `Display` by default, unless overridden by the custom_display attribute. 
  - Conditionally, based on the underlying data type, traits like `Copy`, `Eq`, `Ord`, `Hash` may also be implemented. For primitive data types like `i32` or `bool`, these additional traits will be automatically included.
  - Numeric types, both integer and floating-point, also implement constants `MIN`, `MAX`, `INFINITY`, `NEG_INFINITY`, and `ZERO`. Additionally, for floating-point types, `NAN` is implemented.

- **Attributes:** 
  - Adding the following attributes to `#[strong_type(...)]` allows for additional features:
    - `auto_operators`: Automatically implements relevant arithmetic (for numeric types) or logical (for boolean types) operators.
    - `custom_display`: Allows users to manually implement the `Display` trait, providing an alternative to the default display format.
    - `conversion`: Automatically implements `From` and `Into` traits for the underlying type. This is optional since conversion may make strong types less distinct.
    - `underlying`: Specifies the underlying primitive type for nested strong types.

## Installation
Add `strong-type` to your `Cargo.toml`:
```toml
[dependencies]
strong-type = "0.9"
```

## Supported underlying types:
  - Integer types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
  - Unsigned integer types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
  - Floating-point types: `f32`, `f64`
  - Boolean type: `bool`
  - `char`
  - `String`
  - Strong types of the above types

## Examples
#### Creating a named strong type:
```rust
use strong_type::StrongType;

#[derive(StrongType)]
struct Tag(String);

let tag = Tag::new("dev");
const TAG: Tag = Tag::const_new("prod");
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
use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Nanosecond(u32);

let x = Nanosecond::new(2);
let y = Nanosecond::new(3);
let z = Nanosecond::default();

assert_eq!(x.value(), 2);
assert_eq!(y.value(), 3);
assert_eq!(z.value(), 0);
assert!(x < y);
assert!(y >= x);
assert_eq!(x + y, Nanosecond(5));
```

#### Named bool type with logical operations:

```rust
use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators)]

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
use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(custom_display)]

struct Second(f64);

impl Display for Second {
   fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "Second({:.2})", &self.0)
   }
}

println!("{}", Second::new(std::f64::consts::E)); // "Second(2.72)"
println!("{:?}", Second::new(std::f64::consts::E)); // "Second { value: 2.718281828459045 }"
```

#### Nested strong types:

```rust
#[derive(StrongType)]
#[strong_type(auto_operators)]
struct Dollar(i32);

#[derive(StrongType)]
#[strong_type(auto_operators, underlying = i32)]
struct Cash(Dollar);

#[derive(StrongType)]
#[strong_type(underlying = i32)]
struct Coin(Cash);
```

### Caveats:
- When using `#[derive(StrongType)]`, the traits `Eq` and `PartialEq` are implemented with `impl`. 
 As a result, `StructuralEq` and `StructuralPartialEq` remain unimplemented, preventing pattern matching with strong-typed primitives.