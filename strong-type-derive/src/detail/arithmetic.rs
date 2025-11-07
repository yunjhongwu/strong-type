use super::codegen_framework::{GenerationMode, generate_arithmetic_operators};
use proc_macro2::TokenStream;

pub(crate) fn implement_arithmetic(name: &syn::Ident) -> TokenStream {
    generate_arithmetic_operators(name, GenerationMode::Full)
}
