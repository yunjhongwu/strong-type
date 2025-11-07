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
  - The macro automatically implements `Clone`, `Debug`, `PartialEq`, and `PartialOrd`, and will conditionally add `Copy`, `Default`, `Eq`, `Ord`, and `Hash` when appropriate. `Send` and `Sync` are automatically derived by Rust when the wrapped type implements them.
  - Every generated type exposes ergonomic helpers such as `new`, `const_new`, `into_inner`, `as_ref`, and `as_mut`, plus blanket `AsRef`/`AsMut` implementations so you can seamlessly borrow the inner value.
  - Conditionally, based on the underlying data type, traits like `Copy`, `Eq`, `Ord`, `Hash` may also be implemented. For primitive data types like `i32` or `bool`, these additional traits will be automatically included.
  - Numeric types, both integer and floating-point, also implement constants `MIN`, `MAX`, `INFINITY`, `NEG_INFINITY`, and `ZERO`. Additionally, for floating-point types, `NAN` is implemented.

- **Attributes:**
  - Adding the following attributes to `#[strong_type(...)]` allows for additional features:
    - `auto_operators`: Automatically implements relevant arithmetic (for numeric types) or logical (for boolean types) operators with all ownership variants (owned, `&Self`, etc.).
      - Use `auto_operators = "delegated"` when you want all ownership combinations but prefer smaller binaries (requires the primitive type to be `Copy`); delegated mode routes operator bodies through shared helpers in `strong_type::delegation`, trimming monomorphization in debug builds at the cost of a small inlining opportunity.
      - Use `auto_operators = "minimal"` for a lightweight version that generates only owned-value operations, reducing binary size while maintaining core functionality.
      - Use `auto_operators = "full"` or just `auto_operators` for the complete set of operator implementations.
    - `addable`: Automatically implements the `Add`, `Sub`, and other relevant traits. The attribute is a strict subset of `auto_operators`.
    - `scalable`: Automatically implements the `Mul`, `Div`, `Rem`, and other relevant traits between a strong typed struct and its primitive type. Note that the attribute is not a subset of `auto_operators`.
    - `custom_display`: Allows users to manually implement the `Display` trait, providing an alternative to the default display format.
    - `conversion`: Automatically implements `From`/`Into` for owned and borrowed variants of the underlying type, making it easy to cross the boundary when needed. This is optional since conversion may make strong types less distinct.
    - `underlying`: Specifies the underlying primitive type for nested strong types.

## Installation
Add `strong-type` to your `Cargo.toml`:
```toml
[dependencies]
strong-type = "1.0"
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
With a private field:
```rust
use strong_type::StrongType;

#[derive(StrongType)]
struct Tag(String);

let tag = Tag::new("dev");
const TAG: Tag = Tag::const_new("prod");
```

With a public field:
```rust
use strong_type::StrongType;

#[derive(StrongType)]
struct Timestamp(pub i64);

let timestamp = Timestamp(1701620628123456789);
println!("{}", timestamp); // Timestamp(1701620628123456789)
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

#[derive(StrongType)]
#[strong_type(scalable)]
struct Millisecond(u32);

let x = Millisecond::new(2);

assert_eq!(x * 3, Millisecond(6));
```

#### Minimal operators for reduced binary size:

```rust
use strong_type::StrongType;

// Full mode: generates operators for all ownership combinations
// (Type + Type, Type + &Type, &Type + Type, &Type + &Type)
#[derive(StrongType)]
#[strong_type(auto_operators)]  // or auto_operators = "full"
struct FullPrice(i32);

let x = FullPrice::new(10);
let y = FullPrice::new(20);
assert_eq!(&x + &y, FullPrice(30));  // Works with references

// Minimal mode: generates only owned operations for smaller binary size
#[derive(StrongType)]
#[strong_type(auto_operators = "minimal")]
struct MinimalPrice(i32);

let x = MinimalPrice::new(10);
let y = MinimalPrice::new(20);
assert_eq!(x + y, MinimalPrice(30));  // Works with owned values
// assert_eq!(&x + &y, MinimalPrice(30));  // Won't compile - references not supported

// Minimal mode still supports:
// - All basic operators: +, -, *, /, %
// - Assignment operators: +=, -=, *=, /=, %=
// - Negation: -x
// - Iterator traits: Sum, Product
```

#### Delegated operators for shared codegen

```rust
use strong_type::StrongType;

#[derive(StrongType)]
#[strong_type(auto_operators = "delegated")]
struct DelegatedPrice(i32);

let x = DelegatedPrice::new(10);
let y = DelegatedPrice::new(20);
assert_eq!(&x + &y, DelegatedPrice(30)); // All ownership variants still compile
```

Delegated mode emits the full operator surface but forwards every body to small helpers in `strong_type::delegation`. This keeps ergonomics identical to `auto_operators = "full"` while trimming monomorphization-heavy code, typically shrinking debug binaries by 30-50% versus the full mode. Because those helpers are marked `#[inline(never)]`, expect a tiny throughput regression when micro-benchmarked (<2% in our perf examples), which is usually offset by faster builds and smaller artifacts.

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
- `#[strong_type(scalable)]` does not work for nested strong types.
