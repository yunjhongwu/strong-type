use super::codegen_framework::{GenerationMode, generate_bool_operators};
use proc_macro2::TokenStream;

pub(crate) fn implement_bool_ops(name: &syn::Ident) -> TokenStream {
    generate_bool_operators(name, GenerationMode::Full)
}
