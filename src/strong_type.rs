use crate::detail::{
    custom_display, get_type_group, get_type_ident, implement_arithmetic, implement_basic,
    implement_basic_primitive, implement_basic_string, implement_bool_ops, implement_display,
    implement_hash, implement_min_max, implement_nan, implement_negate, UnderlyingTypeGroup,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(super) fn expand_strong_type(input: DeriveInput, impl_arithmetic: bool) -> TokenStream {
    let name = &input.ident;
    let value_type = get_type_ident(&input);
    let group = get_type_group(value_type);

    let mut ast = quote!();
    ast.extend(implement_basic(name));
    if !custom_display(&input) {
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

    if impl_arithmetic {
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
