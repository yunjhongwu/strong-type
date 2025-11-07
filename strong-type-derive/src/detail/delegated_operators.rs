//! Delegated operator implementations for reduced binary size.
//!
//! This module provides operator implementations that delegate to shared generic
//! functions, reducing monomorphization cost and binary size by 30-50% in debug builds.
//!
//! Delegated mode generates:
//! - All ownership variants for operators
//! - Delegates to shared generic functions in strong_type::delegation
//! - Requires StrongTypeOps trait implementation
//!
//! Trade-off:
//! - Smaller binary size (30-50% reduction)
//! - Slightly reduced optimization potential (functions marked #[inline(never)])
//! - Still zero-cost in most scenarios

use super::codegen_framework::{
    DelegationStrategy, GenerationMode, binary_ops, generate_binary_operator_with_strategy,
    generate_bit_shift_for_type_with_strategy, generate_scalar_operator_with_strategy,
    generate_unary_operator_with_strategy, scalar_ops, unary_ops,
};
use proc_macro2::TokenStream;

/// Generates delegated arithmetic operators (Add, Sub, Mul, Div, Rem) for a type
pub(crate) fn implement_delegated_arithmetic(name: &syn::Ident) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::ADD,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::SUB,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::MUL,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::DIV,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::REM,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));

    result
}

/// Generates delegated Neg operator implementation
pub(crate) fn implement_delegated_negate(name: &syn::Ident) -> TokenStream {
    generate_unary_operator_with_strategy(
        name,
        &unary_ops::NEG,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    )
}

/// Generates delegated boolean operators
pub(crate) fn implement_delegated_bool_ops(name: &syn::Ident) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_unary_operator_with_strategy(
        name,
        &unary_ops::NOT,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::BITAND,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::BITOR,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_binary_operator_with_strategy(
        name,
        &binary_ops::BITXOR,
        GenerationMode::Full,
        DelegationStrategy::Delegate,
    ));

    result
}

/// Generates delegated scalar multiplication and division operators
pub(crate) fn implement_delegated_scalable(
    name: &syn::Ident,
    value_type: &syn::Ident,
) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_scalar_operator_with_strategy(
        name,
        value_type,
        &scalar_ops::MUL,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_scalar_operator_with_strategy(
        name,
        value_type,
        &scalar_ops::DIV,
        DelegationStrategy::Delegate,
    ));
    result.extend(generate_scalar_operator_with_strategy(
        name,
        value_type,
        &scalar_ops::REM,
        DelegationStrategy::Delegate,
    ));

    result
}

/// Generates delegated bit shift operations for all integer types
pub(crate) fn implement_delegated_bit_shift(name: &syn::Ident) -> TokenStream {
    const SHIFT_TYPES: &[&str] = &[
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ];

    let mut result = TokenStream::new();
    for type_str in SHIFT_TYPES {
        let shift_type = syn::parse_str::<syn::Ident>(type_str).unwrap();
        result.extend(generate_bit_shift_for_type_with_strategy(
            name,
            &shift_type,
            DelegationStrategy::Delegate,
        ));
    }
    result
}
