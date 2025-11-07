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
pub trait StrongType: Debug + PartialEq + PartialOrd + Clone {
    type UnderlyingType;
    type PrimitiveType;
}

/// Internal module for operator delegation to reduce binary size.
///
/// This module provides generic helper functions that are shared across all strong types
/// wrapping the same primitive type, reducing monomorphization cost.
#[doc(hidden)]
pub mod delegation {
    /// Trait for accessing the underlying primitive value of a strong type.
    /// This trait is automatically implemented by the StrongType derive macro.
    pub trait StrongTypeOps: Sized {
        type Primitive: Copy;

        /// Extract the primitive value from the strong type
        fn to_primitive(self) -> Self::Primitive;

        /// Create a strong type from a primitive value
        fn from_primitive(val: Self::Primitive) -> Self;
    }

    // ============================================================================
    // Binary Operators - Shared across all strong types per primitive
    // ============================================================================

    /// Shared add implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_add<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Add<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() + rhs.to_primitive())
    }

    /// Shared sub implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_sub<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Sub<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() - rhs.to_primitive())
    }

    /// Shared mul implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_mul<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Mul<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() * rhs.to_primitive())
    }

    /// Shared div implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_div<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Div<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() / rhs.to_primitive())
    }

    /// Shared rem implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_rem<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Rem<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() % rhs.to_primitive())
    }

    /// Shared bitand implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_bitand<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::BitAnd<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() & rhs.to_primitive())
    }

    /// Shared bitor implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_bitor<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::BitOr<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() | rhs.to_primitive())
    }

    /// Shared bitxor implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_bitxor<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::BitXor<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() ^ rhs.to_primitive())
    }

    // ============================================================================
    // Unary Operators
    // ============================================================================

    /// Shared neg implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_neg<T>(val: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Neg<Output = T::Primitive>,
    {
        T::from_primitive(-val.to_primitive())
    }

    /// Shared not implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_not<T>(val: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Not<Output = T::Primitive>,
    {
        T::from_primitive(!val.to_primitive())
    }

    // ============================================================================
    // Bit Shift Operators
    // ============================================================================

    /// Shared shl implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_shl<T, Rhs>(lhs: T, rhs: Rhs) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Shl<Rhs, Output = T::Primitive>,
        Rhs: Copy,
    {
        T::from_primitive(lhs.to_primitive() << rhs)
    }

    /// Shared shr implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_shr<T, Rhs>(lhs: T, rhs: Rhs) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Shr<Rhs, Output = T::Primitive>,
        Rhs: Copy,
    {
        T::from_primitive(lhs.to_primitive() >> rhs)
    }

    // ============================================================================
    // Scalar Operations
    // ============================================================================

    /// Shared scalar mul implementation
    #[inline(never)]
    pub fn delegate_scalar_mul<T>(lhs: T, rhs: T::Primitive) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Mul<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() * rhs)
    }

    /// Shared scalar div implementation
    #[inline(never)]
    pub fn delegate_scalar_div<T>(lhs: T, rhs: T::Primitive) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Div<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() / rhs)
    }

    /// Shared scalar rem implementation
    #[inline(never)]
    pub fn delegate_scalar_rem<T>(lhs: T, rhs: T::Primitive) -> T
    where
        T: StrongTypeOps,
        T::Primitive: std::ops::Rem<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() % rhs)
    }
}
