use super::codegen_framework::{GenerationMode, generate_negation};
use proc_macro2::TokenStream;

pub(crate) fn implement_negate(name: &syn::Ident) -> TokenStream {
    generate_negation(name, GenerationMode::Full)
}
