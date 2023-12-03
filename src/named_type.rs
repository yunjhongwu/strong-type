use crate::detail::{
    custom_display, get_type, get_type_ident, implement_arithmetic, implement_basic,
    implement_basic_primitive, implement_basic_string, implement_display, implement_hash,
    implement_min_max, implement_nan, implement_negate, implement_not, UnderlyingType,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(super) fn expand_named_type(input: DeriveInput, impl_arithmetic: bool) -> TokenStream {
    let name = &input.ident;
    let value_type = get_type_ident(&input);
    let group = get_type(value_type);

    let mut ast = quote!();
    ast.extend(implement_basic(name, value_type));
    if !custom_display(&input) {
        ast.extend(implement_display(name));
    };

    match &group {
        UnderlyingType::Int => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_min_max(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingType::UInt => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_min_max(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingType::Float => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_min_max(name, value_type));
            ast.extend(implement_nan(name, value_type));
        }
        UnderlyingType::Bool => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_not(name));
            ast.extend(implement_hash(name));
        }
        UnderlyingType::Char => {
            ast.extend(implement_basic_primitive(name, value_type));
            ast.extend(implement_hash(name));
        }
        UnderlyingType::String => {
            ast.extend(implement_basic_string(name));
            ast.extend(implement_hash(name));
        }
    }

    if impl_arithmetic {
        match &group {
            UnderlyingType::Int => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
            }
            UnderlyingType::UInt => {
                ast.extend(implement_arithmetic(name));
            }
            UnderlyingType::Float => {
                ast.extend(implement_arithmetic(name));
                ast.extend(implement_negate(name));
            }
            _ => panic!("Non-arithmetic type {value_type}"),
        }
    }

    ast
}
