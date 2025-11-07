use super::codegen_framework::{GenerationMode, generate_all_bit_operations};
use proc_macro2::TokenStream;

pub(crate) fn implement_bit_shift(name: &syn::Ident) -> TokenStream {
    generate_all_bit_operations(name, GenerationMode::Full)
}
