#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

// Re-export String for convenience in no_std + alloc environments
#[cfg(feature = "alloc")]
pub use alloc::string::String;

use core::fmt::Debug;

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
    use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

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
        T::Primitive: Add<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() + rhs.to_primitive())
    }

    /// Shared sub implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_sub<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Sub<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() - rhs.to_primitive())
    }

    /// Shared mul implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_mul<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Mul<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() * rhs.to_primitive())
    }

    /// Shared div implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_div<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Div<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() / rhs.to_primitive())
    }

    /// Shared rem implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_rem<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Rem<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() % rhs.to_primitive())
    }

    /// Shared bitand implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_bitand<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: BitAnd<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() & rhs.to_primitive())
    }

    /// Shared bitor implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_bitor<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: BitOr<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() | rhs.to_primitive())
    }

    /// Shared bitxor implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_bitxor<T>(lhs: T, rhs: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: BitXor<Output = T::Primitive>,
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
        T::Primitive: Neg<Output = T::Primitive>,
    {
        T::from_primitive(-val.to_primitive())
    }

    /// Shared not implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_not<T>(val: T) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Not<Output = T::Primitive>,
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
        T::Primitive: Shl<Rhs, Output = T::Primitive>,
        Rhs: Copy,
    {
        T::from_primitive(lhs.to_primitive() << rhs)
    }

    /// Shared shr implementation - monomorphized once per primitive type
    #[inline(never)]
    pub fn delegate_shr<T, Rhs>(lhs: T, rhs: Rhs) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Shr<Rhs, Output = T::Primitive>,
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
        T::Primitive: Mul<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() * rhs)
    }

    /// Shared scalar div implementation
    #[inline(never)]
    pub fn delegate_scalar_div<T>(lhs: T, rhs: T::Primitive) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Div<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() / rhs)
    }

    /// Shared scalar rem implementation
    #[inline(never)]
    pub fn delegate_scalar_rem<T>(lhs: T, rhs: T::Primitive) -> T
    where
        T: StrongTypeOps,
        T::Primitive: Rem<Output = T::Primitive>,
    {
        T::from_primitive(lhs.to_primitive() % rhs)
    }
}
