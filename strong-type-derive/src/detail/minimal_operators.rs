//! Minimal operator implementations for reduced binary size.
//!
//! This module provides minimal operator implementations that only generate
//! the most commonly used ownership variants, reducing binary size by ~75%
//! compared to full operator implementations.
//!
//! Minimal mode generates:
//! - `impl Op<Self> for Type` (owned + owned)
//! - `impl OpAssign<Self> for Type` (assignment operators)
//! - Sum/Product iterator traits where applicable
//!
//! It does NOT generate:
//! - Reference variants (`&Self`, `&Type`)
//! - Bit shift operations
//! - Scalar operations (those are in scalable module)

use proc_macro2::TokenStream;
use quote::quote;

/// Generates minimal Add operator implementation (owned + owned only).
pub(crate) fn impl_minimal_add(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Add<Self> for #name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.value() + rhs.value())
            }
        }

        impl std::ops::AddAssign<Self> for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.value()
            }
        }

        impl std::iter::Sum<Self> for #name {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, std::ops::Add::add)
            }
        }
    }
}

/// Generates minimal Sub operator implementation (owned - owned only).
pub(crate) fn impl_minimal_sub(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Sub<Self> for #name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.value() - rhs.value())
            }
        }

        impl std::ops::SubAssign<Self> for #name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.value()
            }
        }
    }
}

/// Generates minimal Mul operator implementation (owned * owned only).
pub(crate) fn impl_minimal_mul(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Mul<Self> for #name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.value() * rhs.value())
            }
        }

        impl std::ops::MulAssign<Self> for #name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.value()
            }
        }

        impl std::iter::Product<Self> for #name {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, std::ops::Mul::mul)
            }
        }
    }
}

/// Generates minimal Div operator implementation (owned / owned only).
pub(crate) fn impl_minimal_div(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Div<Self> for #name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.value() / rhs.value())
            }
        }

        impl std::ops::DivAssign<Self> for #name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.value()
            }
        }
    }
}

/// Generates minimal Rem operator implementation (owned % owned only).
pub(crate) fn impl_minimal_rem(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Rem<Self> for #name {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self::new(self.value() % rhs.value())
            }
        }

        impl std::ops::RemAssign<Self> for #name {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.value()
            }
        }
    }
}

/// Generates minimal Neg operator implementation.
pub(crate) fn impl_minimal_negate(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Neg for #name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.value())
            }
        }
    }
}

/// Generates minimal arithmetic operators for integer and float types.
/// Only generates owned + owned variants.
pub(crate) fn implement_minimal_arithmetic(name: &syn::Ident) -> TokenStream {
    let add = impl_minimal_add(name);
    let sub = impl_minimal_sub(name);
    let mul = impl_minimal_mul(name);
    let div = impl_minimal_div(name);
    let rem = impl_minimal_rem(name);

    quote! {
        #add
        #sub
        #mul
        #div
        #rem
    }
}

/// Generates minimal boolean operators.
pub(crate) fn implement_minimal_bool_ops(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::BitAnd<Self> for #name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::new(self.value() & rhs.value())
            }
        }

        impl std::ops::BitAndAssign<Self> for #name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.value()
            }
        }

        impl std::ops::BitOr<Self> for #name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::new(self.value() | rhs.value())
            }
        }

        impl std::ops::BitOrAssign<Self> for #name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.value()
            }
        }

        impl std::ops::BitXor<Self> for #name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self::new(self.value() ^ rhs.value())
            }
        }

        impl std::ops::BitXorAssign<Self> for #name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.value()
            }
        }

        impl std::ops::Not for #name {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self::new(!self.value())
            }
        }
    }
}
