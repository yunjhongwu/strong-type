//! Helper functions for generating operator trait implementations.
//!
//! This module provides reusable functions that eliminate code duplication across
//! operator trait implementations. Instead of manually writing the same pattern
//! for each operator (4 ownership variants + assignment operators), we can now
//! use a single function call.

use proc_macro2::TokenStream;
use quote::quote;

/// Generates a binary operator implementation with all ownership variants.
///
/// This function generates:
/// - `impl Op<Self> for Type`
/// - `impl Op<&Self> for Type`
/// - `impl Op<Type> for &Type`
/// - `impl Op<&Type> for &Type`
/// - `impl OpAssign<Self> for Type`
/// - `impl OpAssign<&Self> for Type`
///
/// # Arguments
/// * `name` - The type identifier
/// * `trait_name` - The operator trait path (e.g., `std::ops::Add`)
/// * `method_name` - The method identifier (e.g., `add`)
/// * `op_token` - The operator token for the operation
/// * `assign_trait_name` - The assignment trait path (e.g., `std::ops::AddAssign`)
/// * `assign_method_name` - The assignment method identifier (e.g., `add_assign`)
/// * `assign_op_token` - The compound assignment operator token (e.g., `+=`)
pub(crate) fn impl_binary_op(
    name: &syn::Ident,
    trait_name: TokenStream,
    method_name: &syn::Ident,
    op_token: TokenStream,
    assign_trait_name: TokenStream,
    assign_method_name: &syn::Ident,
    assign_op_token: TokenStream,
) -> TokenStream {
    quote! {
        impl #trait_name<Self> for #name {
            type Output = Self;

            fn #method_name(self, rhs: Self) -> Self::Output {
                Self::new(self.value() #op_token rhs.value())
            }
        }

        impl #trait_name<&Self> for #name {
            type Output = Self;

            fn #method_name(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() #op_token rhs.value())
            }
        }

        impl<'a> #trait_name<#name> for &'a #name {
            type Output = #name;

            fn #method_name(self, rhs: #name) -> Self::Output {
                #name::new(self.value() #op_token rhs.value())
            }
        }

        impl<'a> #trait_name<&#name> for &'a #name {
            type Output = #name;

            fn #method_name(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() #op_token rhs.value())
            }
        }

        impl #assign_trait_name<Self> for #name {
            fn #assign_method_name(&mut self, rhs: Self) {
                self.0 #assign_op_token rhs.value()
            }
        }

        impl #assign_trait_name<&Self> for #name {
            fn #assign_method_name(&mut self, rhs: &Self) {
                self.0 #assign_op_token rhs.value()
            }
        }
    }
}

/// Generates Add operator implementation with Sum iterator trait.
pub(crate) fn impl_add(name: &syn::Ident) -> TokenStream {
    let add_ident = syn::Ident::new("add", name.span());
    let add_assign_ident = syn::Ident::new("add_assign", name.span());

    let basic_impl = impl_binary_op(
        name,
        quote! { std::ops::Add },
        &add_ident,
        quote! { + },
        quote! { std::ops::AddAssign },
        &add_assign_ident,
        quote! { += },
    );

    quote! {
        #basic_impl

        impl std::iter::Sum<Self> for #name {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, std::ops::Add::add)
            }
        }

        impl<'a> std::iter::Sum<&'a Self> for #name {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, std::ops::Add::add)
            }
        }
    }
}

/// Generates Sub operator implementation.
pub(crate) fn impl_sub(name: &syn::Ident) -> TokenStream {
    let sub_ident = syn::Ident::new("sub", name.span());
    let sub_assign_ident = syn::Ident::new("sub_assign", name.span());

    impl_binary_op(
        name,
        quote! { std::ops::Sub },
        &sub_ident,
        quote! { - },
        quote! { std::ops::SubAssign },
        &sub_assign_ident,
        quote! { -= },
    )
}

/// Generates Mul operator implementation with Product iterator trait.
pub(crate) fn impl_mul(name: &syn::Ident) -> TokenStream {
    let mul_ident = syn::Ident::new("mul", name.span());
    let mul_assign_ident = syn::Ident::new("mul_assign", name.span());

    let basic_impl = impl_binary_op(
        name,
        quote! { std::ops::Mul },
        &mul_ident,
        quote! { * },
        quote! { std::ops::MulAssign },
        &mul_assign_ident,
        quote! { *= },
    );

    quote! {
        #basic_impl

        impl std::iter::Product<Self> for #name {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, std::ops::Mul::mul)
            }
        }

        impl<'a> std::iter::Product<&'a Self> for #name {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, std::ops::Mul::mul)
            }
        }
    }
}

/// Generates Div operator implementation.
pub(crate) fn impl_div(name: &syn::Ident) -> TokenStream {
    let div_ident = syn::Ident::new("div", name.span());
    let div_assign_ident = syn::Ident::new("div_assign", name.span());

    impl_binary_op(
        name,
        quote! { std::ops::Div },
        &div_ident,
        quote! { / },
        quote! { std::ops::DivAssign },
        &div_assign_ident,
        quote! { /= },
    )
}

/// Generates Rem operator implementation.
pub(crate) fn impl_rem(name: &syn::Ident) -> TokenStream {
    let rem_ident = syn::Ident::new("rem", name.span());
    let rem_assign_ident = syn::Ident::new("rem_assign", name.span());

    impl_binary_op(
        name,
        quote! { std::ops::Rem },
        &rem_ident,
        quote! { % },
        quote! { std::ops::RemAssign },
        &rem_assign_ident,
        quote! { %= },
    )
}

/// Generates BitAnd operator implementation.
pub(crate) fn impl_bitand(name: &syn::Ident) -> TokenStream {
    let bitand_ident = syn::Ident::new("bitand", name.span());
    let bitand_assign_ident = syn::Ident::new("bitand_assign", name.span());

    impl_binary_op(
        name,
        quote! { std::ops::BitAnd },
        &bitand_ident,
        quote! { & },
        quote! { std::ops::BitAndAssign },
        &bitand_assign_ident,
        quote! { &= },
    )
}

/// Generates BitOr operator implementation.
pub(crate) fn impl_bitor(name: &syn::Ident) -> TokenStream {
    let bitor_ident = syn::Ident::new("bitor", name.span());
    let bitor_assign_ident = syn::Ident::new("bitor_assign", name.span());

    impl_binary_op(
        name,
        quote! { std::ops::BitOr },
        &bitor_ident,
        quote! { | },
        quote! { std::ops::BitOrAssign },
        &bitor_assign_ident,
        quote! { |= },
    )
}

/// Generates BitXor operator implementation.
pub(crate) fn impl_bitxor(name: &syn::Ident) -> TokenStream {
    let bitxor_ident = syn::Ident::new("bitxor", name.span());
    let bitxor_assign_ident = syn::Ident::new("bitxor_assign", name.span());

    impl_binary_op(
        name,
        quote! { std::ops::BitXor },
        &bitxor_ident,
        quote! { ^ },
        quote! { std::ops::BitXorAssign },
        &bitxor_assign_ident,
        quote! { ^= },
    )
}

/// Generates scalar multiplication operator implementation.
///
/// This generates implementations for:
/// - `Type * scalar`
/// - `&Type * scalar`
/// - `scalar * Type` (commutative)
/// - `scalar * &Type` (commutative)
/// - `Type *= scalar`
pub(crate) fn impl_scalar_mul(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Mul<#value_type> for #name {
            type Output = Self;
            fn mul(self, rhs: #value_type) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl<'a> std::ops::Mul<#value_type> for &'a #name {
            type Output = #name;
            fn mul(self, rhs: #value_type) -> Self::Output {
                #name(self.0 * rhs)
            }
        }

        impl std::ops::Mul<#name> for #value_type {
            type Output = #name;
            fn mul(self, rhs: #name) -> Self::Output {
                #name(self * rhs.0)
            }
        }

        impl<'a> std::ops::Mul<&#name> for #value_type {
            type Output = #name;
            fn mul(self, rhs: &#name) -> Self::Output {
                #name(self * rhs.0)
            }
        }

        impl std::ops::MulAssign<#value_type> for #name {
            fn mul_assign(&mut self, rhs: #value_type) {
                self.0 *= rhs;
            }
        }
    }
}

/// Generates scalar division operator implementation.
///
/// This generates implementations for:
/// - `Type / scalar`
/// - `&Type / scalar`
/// - `Type /= scalar`
pub(crate) fn impl_scalar_div(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Div<#value_type> for #name {
            type Output = Self;
            fn div(self, rhs: #value_type) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl<'a> std::ops::Div<#value_type> for &'a #name {
            type Output = #name;
            fn div(self, rhs: #value_type) -> Self::Output {
                #name(self.0 / rhs)
            }
        }

        impl std::ops::DivAssign<#value_type> for #name {
            fn div_assign(&mut self, rhs: #value_type) {
                self.0 /= rhs;
            }
        }
    }
}

/// Generates scalar remainder operator implementation.
///
/// This generates implementations for:
/// - `Type % scalar`
/// - `&Type % scalar`
/// - `Type %= scalar`
pub(crate) fn impl_scalar_rem(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Rem<#value_type> for #name {
            type Output = Self;
            fn rem(self, rhs: #value_type) -> Self::Output {
                Self(self.0 % rhs)
            }
        }

        impl<'a> std::ops::Rem<#value_type> for &'a #name {
            type Output = #name;
            fn rem(self, rhs: #value_type) -> Self::Output {
                #name(self.0 % rhs)
            }
        }

        impl std::ops::RemAssign<#value_type> for #name {
            fn rem_assign(&mut self, rhs: #value_type) {
                self.0 %= rhs;
            }
        }
    }
}
