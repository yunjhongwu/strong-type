use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic_primitive(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn new(value: #value_type) -> Self {
                Self(value.into())
            }

            pub fn value(&self) -> #value_type {
                self.0
            }
        }

        impl Copy for #name {}

        impl Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }
    }
}
