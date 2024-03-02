use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_conversion(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl From<#value_type> for #name {
            fn from(value: #value_type) -> Self {
                Self::new(value)
            }
        }

        impl From<#name> for #value_type {
            fn from(value: #name) -> #value_type {
                value.0
            }
        }
    }
}

pub(crate) fn implement_str_conversion(name: &syn::Ident) -> TokenStream {
    quote! {
        impl From<&str> for #name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }
    }
}
