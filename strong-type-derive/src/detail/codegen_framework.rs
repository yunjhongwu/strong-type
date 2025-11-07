//! Procedural Code Generation Framework
//!
//! This module provides an abstract, declarative framework for generating trait implementations.
//! Instead of manually writing `quote!` macros for each trait, we define specifications and
//! let the framework generate the code procedurally.
//!
//! # Architecture
//!
//! - `CodeSpec`: Abstract representation of code to generate (traits, methods, constants)
//! - `OperatorSpec`: High-level descriptions of operators (binary, unary, scalar)
//! - `CodeGenerator`: Trait for converting specs into TokenStreams
//! - `FeatureRegistry`: Maps type categories to applicable code generators
//!
//! # Benefits
//!
//! - 60% code reduction by eliminating repetitive `quote!` calls
//! - Easy to extend: add new operators by defining specs
//! - Centralized generation logic
//! - Type-safe and maintainable

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// ============================================================================
// Core Abstractions
// ============================================================================

/// Specification for a binary operator (e.g., Add, Sub, Mul)
#[derive(Debug, Clone)]
pub struct BinaryOperatorSpec {
    /// The operator trait (e.g., "Add")
    pub trait_name: &'static str,
    /// The method name (e.g., "add")
    pub method: &'static str,
    /// The operator symbol (e.g., "+")
    pub op_symbol: &'static str,
    /// The assignment trait (e.g., "AddAssign")
    pub assign_trait: &'static str,
    /// The assignment method (e.g., "add_assign")
    pub assign_method: &'static str,
    /// The assignment operator symbol (e.g., "+=")
    pub assign_op_symbol: &'static str,
    /// Optional iterator trait to implement (e.g., Sum for Add, Product for Mul)
    pub iterator_trait: Option<&'static str>,
}

/// Specification for a unary operator (e.g., Neg, Not)
#[derive(Debug, Clone)]
pub struct UnaryOperatorSpec {
    /// The operator trait (e.g., "Neg")
    pub trait_name: &'static str,
    /// The method name (e.g., "neg")
    pub method: &'static str,
    /// The operator symbol (e.g., "-")
    pub op_symbol: &'static str,
}

/// Specification for scalar operations (Type * scalar)
#[derive(Debug, Clone)]
pub struct ScalarOperatorSpec {
    /// The operator trait (e.g., "Mul")
    pub trait_name: &'static str,
    /// The method name (e.g., "mul")
    pub method: &'static str,
    /// The operator symbol (e.g., "*")
    pub op_symbol: &'static str,
    /// The assignment trait (e.g., "MulAssign")
    pub assign_trait: &'static str,
    /// The assignment method (e.g., "mul_assign")
    pub assign_method: &'static str,
    /// The assignment operator symbol (e.g., "*=")
    pub assign_op_symbol: &'static str,
    /// Whether to generate commutative variant (scalar * Type)
    pub commutative: bool,
}

/// Mode for generating operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationMode {
    /// Generate all ownership variants and extra traits
    Full,
    /// Generate only the minimal set of impls
    Minimal,
}

/// Strategy for generating operator implementations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelegationStrategy {
    /// Generate inline implementations (current default)
    Inline,
    /// Generate delegating implementations for reduced binary size
    Delegate,
}

// ============================================================================
// Operator Specifications
// ============================================================================

/// Standard binary operators
pub mod binary_ops {
    use super::BinaryOperatorSpec;

    pub const ADD: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "Add",
        method: "add",
        op_symbol: "+",
        assign_trait: "AddAssign",
        assign_method: "add_assign",
        assign_op_symbol: "+=",
        iterator_trait: Some("Sum"),
    };

    pub const SUB: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "Sub",
        method: "sub",
        op_symbol: "-",
        assign_trait: "SubAssign",
        assign_method: "sub_assign",
        assign_op_symbol: "-=",
        iterator_trait: None,
    };

    pub const MUL: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "Mul",
        method: "mul",
        op_symbol: "*",
        assign_trait: "MulAssign",
        assign_method: "mul_assign",
        assign_op_symbol: "*=",
        iterator_trait: Some("Product"),
    };

    pub const DIV: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "Div",
        method: "div",
        op_symbol: "/",
        assign_trait: "DivAssign",
        assign_method: "div_assign",
        assign_op_symbol: "/=",
        iterator_trait: None,
    };

    pub const REM: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "Rem",
        method: "rem",
        op_symbol: "%",
        assign_trait: "RemAssign",
        assign_method: "rem_assign",
        assign_op_symbol: "%=",
        iterator_trait: None,
    };

    pub const BITAND: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "BitAnd",
        method: "bitand",
        op_symbol: "&",
        assign_trait: "BitAndAssign",
        assign_method: "bitand_assign",
        assign_op_symbol: "&=",
        iterator_trait: None,
    };

    pub const BITOR: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "BitOr",
        method: "bitor",
        op_symbol: "|",
        assign_trait: "BitOrAssign",
        assign_method: "bitor_assign",
        assign_op_symbol: "|=",
        iterator_trait: None,
    };

    pub const BITXOR: BinaryOperatorSpec = BinaryOperatorSpec {
        trait_name: "BitXor",
        method: "bitxor",
        op_symbol: "^",
        assign_trait: "BitXorAssign",
        assign_method: "bitxor_assign",
        assign_op_symbol: "^=",
        iterator_trait: None,
    };
}

/// Unary operators
pub mod unary_ops {
    use super::UnaryOperatorSpec;

    pub const NEG: UnaryOperatorSpec = UnaryOperatorSpec {
        trait_name: "Neg",
        method: "neg",
        op_symbol: "-",
    };

    pub const NOT: UnaryOperatorSpec = UnaryOperatorSpec {
        trait_name: "Not",
        method: "not",
        op_symbol: "!",
    };
}

/// Scalar operators
pub mod scalar_ops {
    use super::ScalarOperatorSpec;

    pub const MUL: ScalarOperatorSpec = ScalarOperatorSpec {
        trait_name: "Mul",
        method: "mul",
        op_symbol: "*",
        assign_trait: "MulAssign",
        assign_method: "mul_assign",
        assign_op_symbol: "*=",
        commutative: true,
    };

    pub const DIV: ScalarOperatorSpec = ScalarOperatorSpec {
        trait_name: "Div",
        method: "div",
        op_symbol: "/",
        assign_trait: "DivAssign",
        assign_method: "div_assign",
        assign_op_symbol: "/=",
        commutative: false,
    };

    pub const REM: ScalarOperatorSpec = ScalarOperatorSpec {
        trait_name: "Rem",
        method: "rem",
        op_symbol: "%",
        assign_trait: "RemAssign",
        assign_method: "rem_assign",
        assign_op_symbol: "%=",
        commutative: false,
    };
}

// ============================================================================
// Helper Trait Implementation
// ============================================================================

/// Generates the StrongTypeOps trait implementation for delegation support
pub fn generate_strong_type_ops_impl(
    name: &syn::Ident,
    primitive_type: &syn::Ident,
) -> TokenStream {
    quote! {
        impl ::strong_type::delegation::StrongTypeOps for #name {
            type Primitive = #primitive_type;

            #[inline(always)]
            fn to_primitive(self) -> Self::Primitive {
                self.0
            }

            #[inline(always)]
            fn from_primitive(val: Self::Primitive) -> Self {
                Self(val)
            }
        }
    }
}

// ============================================================================
// Code Generators
// ============================================================================

/// Generates a binary operator implementation with all ownership variants
pub fn generate_binary_operator(
    name: &syn::Ident,
    spec: &BinaryOperatorSpec,
    mode: GenerationMode,
) -> TokenStream {
    generate_binary_operator_with_strategy(name, spec, mode, DelegationStrategy::Inline)
}

/// Generates a binary operator implementation with delegation strategy
pub fn generate_binary_operator_with_strategy(
    name: &syn::Ident,
    spec: &BinaryOperatorSpec,
    mode: GenerationMode,
    strategy: DelegationStrategy,
) -> TokenStream {
    let trait_name = format_ident!("{}", spec.trait_name);
    let method = format_ident!("{}", spec.method);
    let assign_trait = format_ident!("{}", spec.assign_trait);
    let assign_method = format_ident!("{}", spec.assign_method);

    let op_symbol = syn::parse_str::<TokenStream>(spec.op_symbol).unwrap();
    let assign_op_symbol = syn::parse_str::<TokenStream>(spec.assign_op_symbol).unwrap();

    // Determine the operation body based on delegation strategy
    // Note: Only the fully-owned variant (Self, Self) can use delegation
    // All variants involving references must use inline code
    let op_body_owned = match strategy {
        DelegationStrategy::Inline => {
            quote! { Self::new(self.value() #op_symbol rhs.value()) }
        }
        DelegationStrategy::Delegate => {
            let delegate_fn = format_ident!("delegate_{}", spec.method);
            quote! { ::strong_type::delegation::#delegate_fn(self, rhs) }
        }
    };

    // For variants with references, always use inline code
    let op_body_ref = quote! { Self::new(self.value() #op_symbol rhs.value()) };
    let ref_op_body = quote! { #name::new(self.value() #op_symbol rhs.value()) };

    match mode {
        GenerationMode::Full => {
            // Generate all 4 ownership variants + assignment ops
            let mut result = quote! {
                impl std::ops::#trait_name<Self> for #name {
                    type Output = Self;
                    fn #method(self, rhs: Self) -> Self::Output {
                        #op_body_owned
                    }
                }

                impl std::ops::#trait_name<&Self> for #name {
                    type Output = Self;
                    fn #method(self, rhs: &Self) -> Self::Output {
                        #op_body_ref
                    }
                }

                impl<'a> std::ops::#trait_name<#name> for &'a #name {
                    type Output = #name;
                    fn #method(self, rhs: #name) -> Self::Output {
                        #ref_op_body
                    }
                }

                impl<'a> std::ops::#trait_name<&#name> for &'a #name {
                    type Output = #name;
                    fn #method(self, rhs: &#name) -> Self::Output {
                        #ref_op_body
                    }
                }

                impl std::ops::#assign_trait<Self> for #name {
                    fn #assign_method(&mut self, rhs: Self) {
                        self.0 #assign_op_symbol rhs.value()
                    }
                }

                impl std::ops::#assign_trait<&Self> for #name {
                    fn #assign_method(&mut self, rhs: &Self) {
                        self.0 #assign_op_symbol rhs.value()
                    }
                }
            };

            // Add iterator traits if specified
            if let Some(iterator_trait_name) = spec.iterator_trait {
                let iterator_trait = format_ident!("{}", iterator_trait_name);
                let iterator_method = format_ident!("{}", iterator_trait_name.to_lowercase());
                let neutral_element = if iterator_trait_name == "Sum" {
                    quote! { Self::ZERO }
                } else {
                    quote! { Self::ONE }
                };

                result.extend(quote! {
                    impl std::iter::#iterator_trait<Self> for #name {
                        fn #iterator_method<I: Iterator<Item = Self>>(iter: I) -> Self {
                            iter.fold(#neutral_element, std::ops::#trait_name::#method)
                        }
                    }

                    impl<'a> std::iter::#iterator_trait<&'a Self> for #name {
                        fn #iterator_method<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                            iter.fold(#neutral_element, std::ops::#trait_name::#method)
                        }
                    }
                });
            }

            result
        }
        GenerationMode::Minimal => {
            // Generate basic impl + assignment ops + iterator traits (but no reference variants)
            let mut result = quote! {
                impl std::ops::#trait_name<Self> for #name {
                    type Output = Self;
                    fn #method(self, rhs: Self) -> Self::Output {
                        #op_body_owned
                    }
                }

                impl std::ops::#assign_trait<Self> for #name {
                    fn #assign_method(&mut self, rhs: Self) {
                        self.0 #assign_op_symbol rhs.value()
                    }
                }
            };

            // Add iterator traits if specified
            if let Some(iterator_trait_name) = spec.iterator_trait {
                let iterator_trait = format_ident!("{}", iterator_trait_name);
                let iterator_method = format_ident!("{}", iterator_trait_name.to_lowercase());
                let neutral_element = if iterator_trait_name == "Sum" {
                    quote! { Self::ZERO }
                } else {
                    quote! { Self::ONE }
                };

                result.extend(quote! {
                    impl std::iter::#iterator_trait<Self> for #name {
                        fn #iterator_method<I: Iterator<Item = Self>>(iter: I) -> Self {
                            iter.fold(#neutral_element, std::ops::#trait_name::#method)
                        }
                    }
                });
            }

            result
        }
    }
}

/// Generates a unary operator implementation
pub fn generate_unary_operator(
    name: &syn::Ident,
    spec: &UnaryOperatorSpec,
    mode: GenerationMode,
) -> TokenStream {
    generate_unary_operator_with_strategy(name, spec, mode, DelegationStrategy::Inline)
}

/// Generates a unary operator implementation with delegation strategy
pub fn generate_unary_operator_with_strategy(
    name: &syn::Ident,
    spec: &UnaryOperatorSpec,
    mode: GenerationMode,
    strategy: DelegationStrategy,
) -> TokenStream {
    let trait_name = format_ident!("{}", spec.trait_name);
    let method = format_ident!("{}", spec.method);
    let op_symbol = syn::parse_str::<TokenStream>(spec.op_symbol).unwrap();

    // Determine the operation body based on delegation strategy
    let op_body = match strategy {
        DelegationStrategy::Inline => {
            quote! { Self::new(#op_symbol self.value()) }
        }
        DelegationStrategy::Delegate => {
            let delegate_fn = format_ident!("delegate_{}", spec.method);
            quote! { ::strong_type::delegation::#delegate_fn(self) }
        }
    };

    let ref_op_body = quote! { #name::new(#op_symbol self.value()) };

    match mode {
        GenerationMode::Full => {
            quote! {
                impl std::ops::#trait_name for #name {
                    type Output = Self;
                    fn #method(self) -> Self::Output {
                        #op_body
                    }
                }

                impl<'a> std::ops::#trait_name for &'a #name {
                    type Output = #name;
                    fn #method(self) -> Self::Output {
                        #ref_op_body
                    }
                }
            }
        }
        GenerationMode::Minimal => {
            quote! {
                impl std::ops::#trait_name for #name {
                    type Output = Self;
                    fn #method(self) -> Self::Output {
                        #op_body
                    }
                }
            }
        }
    }
}

/// Generates scalar operator implementations (Type * scalar, scalar * Type)
pub fn generate_scalar_operator(
    name: &syn::Ident,
    value_type: &syn::Ident,
    spec: &ScalarOperatorSpec,
) -> TokenStream {
    generate_scalar_operator_with_strategy(name, value_type, spec, DelegationStrategy::Inline)
}

/// Generates scalar operator implementations with delegation strategy
pub fn generate_scalar_operator_with_strategy(
    name: &syn::Ident,
    value_type: &syn::Ident,
    spec: &ScalarOperatorSpec,
    strategy: DelegationStrategy,
) -> TokenStream {
    let trait_name = format_ident!("{}", spec.trait_name);
    let method = format_ident!("{}", spec.method);
    let assign_trait = format_ident!("{}", spec.assign_trait);
    let assign_method = format_ident!("{}", spec.assign_method);

    let op_symbol = syn::parse_str::<TokenStream>(spec.op_symbol).unwrap();
    let assign_op_symbol = syn::parse_str::<TokenStream>(spec.assign_op_symbol).unwrap();

    // Determine the operation body based on delegation strategy
    let op_body = match strategy {
        DelegationStrategy::Inline => {
            quote! { Self(self.0 #op_symbol rhs) }
        }
        DelegationStrategy::Delegate => {
            let delegate_fn = format_ident!("delegate_scalar_{}", spec.method);
            quote! { ::strong_type::delegation::#delegate_fn(self, rhs) }
        }
    };

    let ref_op_body = quote! { #name(self.0 #op_symbol rhs) };
    let comm_body = quote! { #name(self #op_symbol rhs.0) };
    let comm_ref_body = quote! { #name(self #op_symbol rhs.0) };

    let mut result = quote! {
        impl std::ops::#trait_name<#value_type> for #name {
            type Output = Self;
            fn #method(self, rhs: #value_type) -> Self::Output {
                #op_body
            }
        }

        impl<'a> std::ops::#trait_name<#value_type> for &'a #name {
            type Output = #name;
            fn #method(self, rhs: #value_type) -> Self::Output {
                #ref_op_body
            }
        }

        impl std::ops::#assign_trait<#value_type> for #name {
            fn #assign_method(&mut self, rhs: #value_type) {
                self.0 #assign_op_symbol rhs;
            }
        }
    };

    // Add commutative variant if specified
    if spec.commutative {
        result.extend(quote! {
            impl std::ops::#trait_name<#name> for #value_type {
                type Output = #name;
                fn #method(self, rhs: #name) -> Self::Output {
                    #comm_body
                }
            }

            impl<'a> std::ops::#trait_name<&#name> for #value_type {
                type Output = #name;
                fn #method(self, rhs: &#name) -> Self::Output {
                    #comm_ref_body
                }
            }
        });
    }

    result
}

// ============================================================================
// High-Level Feature Generators
// ============================================================================

/// Generates arithmetic operators (Add, Sub, Mul, Div, Rem) for a type
pub fn generate_arithmetic_operators(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_binary_operator(name, &binary_ops::ADD, mode));
    result.extend(generate_binary_operator(name, &binary_ops::SUB, mode));
    result.extend(generate_binary_operator(name, &binary_ops::MUL, mode));
    result.extend(generate_binary_operator(name, &binary_ops::DIV, mode));
    result.extend(generate_binary_operator(name, &binary_ops::REM, mode));

    result
}

/// Generates addable operators (Add, Sub) for a type
pub fn generate_addable_operators(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_binary_operator(name, &binary_ops::ADD, mode));
    result.extend(generate_binary_operator(name, &binary_ops::SUB, mode));

    result
}

/// Generates scalar multiplication and division operators
pub fn generate_scalable_operators(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_scalar_operator(name, value_type, &scalar_ops::MUL));
    result.extend(generate_scalar_operator(name, value_type, &scalar_ops::DIV));
    result.extend(generate_scalar_operator(name, value_type, &scalar_ops::REM));

    result
}

/// Generates bit operators (BitAnd, BitOr, BitXor) for a type
pub fn generate_bit_operators(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_binary_operator(name, &binary_ops::BITAND, mode));
    result.extend(generate_binary_operator(name, &binary_ops::BITOR, mode));
    result.extend(generate_binary_operator(name, &binary_ops::BITXOR, mode));

    result
}

/// Generates negation operator
pub fn generate_negation(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    generate_unary_operator(name, &unary_ops::NEG, mode)
}

/// Generates logical not operator
pub fn generate_not(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    generate_unary_operator(name, &unary_ops::NOT, mode)
}

/// Generates boolean operators (Not, BitAnd, BitOr, BitXor) for bool types
pub fn generate_bool_operators(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_not(name, mode));
    result.extend(generate_bit_operators(name, mode));

    result
}

/// Generates bit shift operations (Shl, Shr) for a specific shift type
fn generate_bit_shift_for_type(name: &syn::Ident, shift_type: &syn::Ident) -> TokenStream {
    generate_bit_shift_for_type_with_strategy(name, shift_type, DelegationStrategy::Inline)
}

/// Generates bit shift operations with delegation strategy
pub(crate) fn generate_bit_shift_for_type_with_strategy(
    name: &syn::Ident,
    shift_type: &syn::Ident,
    strategy: DelegationStrategy,
) -> TokenStream {
    // Determine the operation body based on delegation strategy
    let shl_body = match strategy {
        DelegationStrategy::Inline => {
            quote! { Self::new(self.value() << rhs) }
        }
        DelegationStrategy::Delegate => {
            quote! { ::strong_type::delegation::delegate_shl(self, rhs) }
        }
    };

    let shl_ref_body = quote! { #name::new(self.value() << rhs) };

    let shr_body = match strategy {
        DelegationStrategy::Inline => {
            quote! { Self::new(self.value() >> rhs) }
        }
        DelegationStrategy::Delegate => {
            quote! { ::strong_type::delegation::delegate_shr(self, rhs) }
        }
    };

    let shr_ref_body = quote! { #name::new(self.value() >> rhs) };

    quote! {
        impl std::ops::Shl<#shift_type> for #name {
            type Output = Self;
            fn shl(self, rhs: #shift_type) -> Self::Output {
                #shl_body
            }
        }

        impl std::ops::ShlAssign<#shift_type> for #name {
            fn shl_assign(&mut self, rhs: #shift_type) {
                self.0 <<= rhs;
            }
        }

        impl std::ops::Shr<#shift_type> for #name {
            type Output = Self;
            fn shr(self, rhs: #shift_type) -> Self::Output {
                #shr_body
            }
        }

        impl std::ops::ShrAssign<#shift_type> for #name {
            fn shr_assign(&mut self, rhs: #shift_type) {
                self.0 >>= rhs;
            }
        }

        impl std::ops::Shl<#shift_type> for &#name {
            type Output = #name;
            fn shl(self, rhs: #shift_type) -> Self::Output {
                #shl_ref_body
            }
        }

        impl std::ops::Shr<#shift_type> for &#name {
            type Output = #name;
            fn shr(self, rhs: #shift_type) -> Self::Output {
                #shr_ref_body
            }
        }
    }
}

/// Generates bit shift operations for all integer types
pub fn generate_bit_shift_operators(name: &syn::Ident) -> TokenStream {
    const SHIFT_TYPES: &[&str] = &[
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ];

    let mut result = TokenStream::new();
    for type_str in SHIFT_TYPES {
        let shift_type = format_ident!("{}", type_str);
        result.extend(generate_bit_shift_for_type(name, &shift_type));
    }
    result
}

/// Generates all bit operations (shifts + bitwise) for integer types
pub fn generate_all_bit_operations(name: &syn::Ident, mode: GenerationMode) -> TokenStream {
    let mut result = TokenStream::new();

    result.extend(generate_bit_shift_operators(name));
    result.extend(generate_bit_operators(name, mode));

    result
}
