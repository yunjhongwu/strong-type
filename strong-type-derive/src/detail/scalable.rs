use super::codegen_framework::generate_scalable_operators;
use proc_macro2::TokenStream;

pub(crate) fn implement_scalable(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    generate_scalable_operators(name, value_type)
}
