use super::codegen_framework::{GenerationMode, generate_addable_operators};
use proc_macro2::TokenStream;

pub(crate) fn implement_addable(name: &syn::Ident) -> TokenStream {
    generate_addable_operators(name, GenerationMode::Full)
}
