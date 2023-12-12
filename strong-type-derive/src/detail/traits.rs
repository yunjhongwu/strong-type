use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_strong_type_trait(
    name: &syn::Ident,
    value_type: &syn::Ident,
) -> TokenStream {
    quote! {
        impl StrongType for #name {
            type UnderlyingType = #value_type;
        }
    }
}
