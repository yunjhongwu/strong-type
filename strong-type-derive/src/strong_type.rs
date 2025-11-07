use crate::detail::{
    AutoOperatorMode, TypeMetadata, UnderlyingType, ValueTypeGroup, impl_minimal_negate,
    implement_addable, implement_arithmetic, implement_basic, implement_basic_primitive,
    implement_basic_string, implement_bit_shift, implement_bool_ops, implement_constants,
    implement_constants_derived, implement_conversion, implement_display, implement_hash,
    implement_infinity, implement_limit, implement_minimal_arithmetic, implement_minimal_bool_ops,
    implement_nan, implement_negate, implement_primitive_accessor,
    implement_primitive_accessor_derived, implement_primitive_str_accessor,
    implement_primitive_str_accessor_derived, implement_scalable, implement_str_conversion,
    validate_struct,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(super) fn expand_strong_type(input: DeriveInput) -> TokenStream {
    match expand_strong_type_impl(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn expand_strong_type_impl(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    validate_struct(&input)?;

    // Resolve all type information once and cache it
    let metadata = TypeMetadata::new(&input)?;

    let name = &metadata.name;
    let value_type = &metadata.value_type;
    let primitive_type = &metadata.primitive_type;
    let type_group = &metadata.type_group;
    let auto_operator_mode = metadata.auto_operator_mode;
    let has_addable = metadata.has_addable;
    let has_scalable = metadata.has_scalable;
    let has_custom_display = metadata.has_custom_display;
    let has_conversion = metadata.has_conversion;

    let mut ast = quote!();
    ast.extend(implement_basic(name, value_type, primitive_type));

    if !has_custom_display {
        ast.extend(implement_display(name));
    };

    if has_conversion {
        ast.extend(implement_conversion(name, value_type));
        if let ValueTypeGroup::String(UnderlyingType::Primitive) = &type_group {
            ast.extend(implement_str_conversion(name));
        }
    }

    match &type_group {
        ValueTypeGroup::Int(underlying_type)
        | ValueTypeGroup::UInt(underlying_type)
        | ValueTypeGroup::Float(underlying_type)
        | ValueTypeGroup::Bool(underlying_type)
        | ValueTypeGroup::Char(underlying_type) => match underlying_type {
            UnderlyingType::Primitive => {
                ast.extend(implement_primitive_accessor(name, value_type))
            }
            UnderlyingType::Derived => {
                ast.extend(implement_primitive_accessor_derived(name, primitive_type))
            }
        },
        ValueTypeGroup::String(UnderlyingType::Primitive) => {
            ast.extend(implement_primitive_str_accessor(name));
        }
        ValueTypeGroup::String(UnderlyingType::Derived) => {
            ast.extend(implement_primitive_str_accessor_derived(name, value_type));
        }
    }

    match &type_group {
        ValueTypeGroup::Int(underlying_type) | ValueTypeGroup::UInt(underlying_type) => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_hash(name));
            ast.extend(implement_limit(name, value_type));
            match underlying_type {
                UnderlyingType::Primitive => ast.extend(implement_constants(name, value_type)),
                UnderlyingType::Derived => {
                    ast.extend(implement_constants_derived(name, value_type))
                }
            }
        }
        ValueTypeGroup::Float(underlying_type) => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_nan(name, value_type));
            ast.extend(implement_limit(name, value_type));
            ast.extend(implement_infinity(name, value_type));
            match underlying_type {
                UnderlyingType::Primitive => {
                    ast.extend(implement_constants(name, value_type));
                }
                UnderlyingType::Derived => {
                    ast.extend(implement_constants_derived(name, value_type));
                }
            }
        }
        ValueTypeGroup::Bool(_) => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_hash(name));
        }
        ValueTypeGroup::Char(_) => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_hash(name));
        }
        ValueTypeGroup::String(_) => {
            ast.extend(implement_basic_string(name));
            ast.extend(implement_hash(name));
        }
    }

    // Consolidate operator implementations in a single match to avoid repeated pattern matching
    match &type_group {
        ValueTypeGroup::Float(_) => {
            match auto_operator_mode {
                AutoOperatorMode::Full => {
                    ast.extend(implement_arithmetic(name));
                    ast.extend(implement_negate(name));
                }
                AutoOperatorMode::Minimal => {
                    ast.extend(implement_minimal_arithmetic(name));
                    ast.extend(impl_minimal_negate(name));
                }
                AutoOperatorMode::None => {
                    if has_addable {
                        ast.extend(implement_addable(name));
                        ast.extend(implement_negate(name));
                    }
                }
            }
            if has_scalable {
                ast.extend(implement_scalable(name, value_type));
                if auto_operator_mode == AutoOperatorMode::None && !has_addable {
                    ast.extend(implement_negate(name));
                }
            }
        }
        ValueTypeGroup::Int(_) => {
            match auto_operator_mode {
                AutoOperatorMode::Full => {
                    ast.extend(implement_arithmetic(name));
                    ast.extend(implement_negate(name));
                    ast.extend(implement_bit_shift(name));
                }
                AutoOperatorMode::Minimal => {
                    ast.extend(implement_minimal_arithmetic(name));
                    ast.extend(impl_minimal_negate(name));
                    // No bit shifts in minimal mode
                }
                AutoOperatorMode::None => {
                    if has_addable {
                        ast.extend(implement_addable(name));
                        ast.extend(implement_negate(name));
                    }
                }
            }
            if has_scalable {
                ast.extend(implement_scalable(name, value_type));
                if auto_operator_mode == AutoOperatorMode::None && !has_addable {
                    ast.extend(implement_negate(name));
                }
            }
        }
        ValueTypeGroup::UInt(_) => {
            match auto_operator_mode {
                AutoOperatorMode::Full => {
                    ast.extend(implement_arithmetic(name));
                    ast.extend(implement_bit_shift(name));
                }
                AutoOperatorMode::Minimal => {
                    ast.extend(implement_minimal_arithmetic(name));
                    // No bit shifts in minimal mode
                }
                AutoOperatorMode::None => {
                    if has_addable {
                        ast.extend(implement_addable(name));
                    }
                }
            }
            if has_scalable {
                ast.extend(implement_scalable(name, value_type));
            }
        }
        ValueTypeGroup::Bool(_) => match auto_operator_mode {
            AutoOperatorMode::Full => {
                ast.extend(implement_bool_ops(name));
            }
            AutoOperatorMode::Minimal => {
                ast.extend(implement_minimal_bool_ops(name));
            }
            AutoOperatorMode::None => {}
        },
        ValueTypeGroup::Char(_) | ValueTypeGroup::String(_) => {}
    }

    Ok(ast)
}
