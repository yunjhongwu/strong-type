use crate::detail::{
    get_attributes, get_type_group, get_type_ident, implement_arithmetic, implement_basic,
    implement_basic_primitive, implement_basic_string, implement_bit_shift, implement_bool_ops,
    implement_constants, implement_display, implement_hash, implement_nan, implement_negate,
    is_struct_valid, StrongTypeAttributes, UnderlyingTypeGroup,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(super) fn expand_strong_type(input: DeriveInput) -> TokenStream {
    if !is_struct_valid(&input) {
        panic!("Strong type must be a tuple struct with one private field.");
    }

    let name = &input.ident;
    let value_type = get_type_ident(&input);
    let group = get_type_group(value_type);
    let StrongTypeAttributes {
        has_auto_operators,
        has_custom_display,
    } = get_attributes(&input);

    let mut ast = quote!();
    ast.extend(implement_basic(name, value_type));

    if !has_custom_display {
        ast.extend(implement_display(name));
    };

    match &group {
        UnderlyingTypeGroup::Int | UnderlyingTypeGroup::UInt => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_constants(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingTypeGroup::Float => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_constants(name, value_type));
            ast.extend(implement_nan(name, value_type));
        }
        UnderlyingTypeGroup::Bool => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingTypeGroup::Char => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingTypeGroup::String => {
            ast.extend(implement_basic_string(name));
            ast.extend(implement_hash(name));
        }
    }

    if has_auto_operators {
        match &group {
            UnderlyingTypeGroup::Float => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
            }
            UnderlyingTypeGroup::Int => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
                ast.extend(implement_bit_shift(name));
            }
            UnderlyingTypeGroup::UInt => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_bit_shift(name));
            }
            UnderlyingTypeGroup::Bool => {
                ast.extend(implement_bool_ops(name));
            }
            _ => {}
        }
    }

    ast
}
