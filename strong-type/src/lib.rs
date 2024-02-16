//! This crate offers a derive macro for crafting strong types in Rust, where a strong type
//! encapsulates a primitive type, providing a distinct, purpose-specific identity. This pattern
//! is useful for creating distinct types for distinct purposes. Several macro attributes are
//! provided to customize the strong type, such as directly implementing arithmetic operators of
//! the underlying primitive type,
//!
//! See the [crate documentation](https://crates.io/crates/strong-type) for more details and examples.
//!

use std::fmt::Debug;

/// Derive macro to create strong types in Rust.
pub use strong_type_derive::StrongType;

/// Trait for strong types to obtain the associated underlying type and primitive type.
pub trait StrongType: Debug + PartialEq + PartialOrd + Clone + Default + Send + Sync {
    type UnderlyingType: Default;
    type PrimitiveType;
}
