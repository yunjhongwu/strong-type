use crate::detail::{
    get_type_group, get_type_ident, has_custom_display, has_numeric, implement_arithmetic,
    implement_basic, implement_basic_primitive, implement_basic_string, implement_bool_ops,
    implement_display, implement_hash, implement_min_max, implement_nan, implement_negate,
    UnderlyingTypeGroup,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Visibility};

fn is_struct_valid(input: &DeriveInput) -> bool {
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            return (fields_unnamed.unnamed.len() == 1)
                && matches!(
                    fields_unnamed.unnamed.first().unwrap().vis,
                    Visibility::Inherited
                );
        }
    }
    false
}

pub(super) fn expand_strong_type(input: DeriveInput) -> TokenStream {
    if !is_struct_valid(&input) {
        panic!("Strong type must be a tuple struct with one private field.");
    }

    let name = &input.ident;
    let value_type = get_type_ident(&input);
    let group = get_type_group(value_type);

    let mut ast = quote!();
    ast.extend(implement_basic(name, value_type));
    if !has_custom_display(&input) {
        ast.extend(implement_display(name));
    };

    match &group {
        UnderlyingTypeGroup::Int | UnderlyingTypeGroup::UInt => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_min_max(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingTypeGroup::Float => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_min_max(name, value_type));
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

    if has_numeric(&input) {
        match &group {
            UnderlyingTypeGroup::Int | UnderlyingTypeGroup::Float => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
            }
            UnderlyingTypeGroup::UInt => {
                ast.extend(implement_arithmetic(name));
            }
            UnderlyingTypeGroup::Bool => {
                ast.extend(implement_bool_ops(name));
            }
            _ => panic!("Non-numeric type: {value_type}"),
        }
    }

    ast
}
