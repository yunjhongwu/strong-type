use crate::detail::{
    get_attributes, get_type, implement_arithmetic, implement_basic, implement_basic_primitive,
    implement_basic_string, implement_bit_shift, implement_bool_ops, implement_constants,
    implement_constants_derived, implement_display, implement_hash, implement_infinity,
    implement_limit, implement_nan, implement_negate, implement_primitive_accessor,
    implement_primitive_accessor_derived, implement_primitive_str_accessor,
    implement_primitive_str_accessor_derived, is_struct_valid, StrongTypeAttributes, TypeInfo,
    UnderlyingType, ValueTypeGroup,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(super) fn expand_strong_type(input: DeriveInput) -> TokenStream {
    if !is_struct_valid(&input) {
        panic!("Strong type must be a tuple struct with one private field.");
    }

    let name = &input.ident;
    let TypeInfo {
        primitive_type,
        value_type,
        type_group,
    } = get_type(&input);
    let StrongTypeAttributes {
        has_auto_operators,
        has_custom_display,
    } = get_attributes(&input);

    let mut ast = quote!();
    ast.extend(implement_basic(name, &value_type));

    if !has_custom_display {
        ast.extend(implement_display(name));
    };

    match &type_group {
        ValueTypeGroup::Int(underlying_type)
        | ValueTypeGroup::UInt(underlying_type)
        | ValueTypeGroup::Float(underlying_type)
        | ValueTypeGroup::Bool(underlying_type)
        | ValueTypeGroup::Char(underlying_type) => match underlying_type {
            UnderlyingType::Primitive => {
                ast.extend(implement_primitive_accessor(name, &value_type))
            }
            UnderlyingType::Derived => {
                ast.extend(implement_primitive_accessor_derived(name, &primitive_type))
            }
        },
        ValueTypeGroup::String(UnderlyingType::Primitive) => {
            ast.extend(implement_primitive_str_accessor(name));
        }
        ValueTypeGroup::String(UnderlyingType::Derived) => {
            ast.extend(implement_primitive_str_accessor_derived(name, &value_type));
        }
    }

    match &type_group {
        ValueTypeGroup::Int(underlying_type) | ValueTypeGroup::UInt(underlying_type) => {
            ast.extend(implement_basic_primitive(name, &value_type));
            ast.extend(implement_hash(name));
            ast.extend(implement_limit(name, &value_type));
            match underlying_type {
                UnderlyingType::Primitive => ast.extend(implement_constants(name, &value_type)),
                UnderlyingType::Derived => {
                    ast.extend(implement_constants_derived(name, &value_type))
                }
            }
        }
        ValueTypeGroup::Float(underlying_type) => {
            ast.extend(implement_basic_primitive(name, &value_type));
            ast.extend(implement_nan(name, &value_type));
            ast.extend(implement_limit(name, &value_type));
            ast.extend(implement_infinity(name, &value_type));
            match underlying_type {
                UnderlyingType::Primitive => {
                    ast.extend(implement_constants(name, &value_type));
                }
                UnderlyingType::Derived => {
                    ast.extend(implement_constants_derived(name, &value_type));
                }
            }
        }
        ValueTypeGroup::Bool(_) => {
            ast.extend(implement_basic_primitive(name, &value_type));
            ast.extend(implement_hash(name));
        }
        ValueTypeGroup::Char(_) => {
            ast.extend(implement_basic_primitive(name, &value_type));
            ast.extend(implement_hash(name));
        }
        ValueTypeGroup::String(_) => {
            ast.extend(implement_basic_string(name));
            ast.extend(implement_hash(name));
        }
    }

    if has_auto_operators {
        match &type_group {
            ValueTypeGroup::Float(_) => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
            }
            ValueTypeGroup::Int(_) => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
                ast.extend(implement_bit_shift(name));
            }
            ValueTypeGroup::UInt(_) => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_bit_shift(name));
            }
            ValueTypeGroup::Bool(_) => {
                ast.extend(implement_bool_ops(name));
            }
            ValueTypeGroup::Char(_) | ValueTypeGroup::String(_) => {}
        }
    }

    ast
}
