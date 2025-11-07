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

        impl<'a> From<&'a #value_type> for #name
        where
            #value_type: Clone,
        {
            fn from(value: &'a #value_type) -> Self {
                Self::new(value.clone())
            }
        }

        impl<'a> From<&'a #name> for &'a #value_type {
            fn from(value: &'a #name) -> Self {
                value.as_ref()
            }
        }

        impl<'a> From<&'a mut #name> for &'a mut #value_type {
            fn from(value: &'a mut #name) -> Self {
                value.as_mut()
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
