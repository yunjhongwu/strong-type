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

use super::codegen_framework::{
    GenerationMode, generate_arithmetic_operators, generate_bool_operators, generate_negation,
};
use proc_macro2::TokenStream;

/// Generates minimal arithmetic operators for integer and float types.
/// Only generates owned + owned variants.
pub(crate) fn implement_minimal_arithmetic(name: &syn::Ident) -> TokenStream {
    generate_arithmetic_operators(name, GenerationMode::Minimal)
}

/// Generates minimal Neg operator implementation.
pub(crate) fn impl_minimal_negate(name: &syn::Ident) -> TokenStream {
    generate_negation(name, GenerationMode::Minimal)
}

/// Generates minimal boolean operators.
pub(crate) fn implement_minimal_bool_ops(name: &syn::Ident) -> TokenStream {
    generate_bool_operators(name, GenerationMode::Minimal)
}
