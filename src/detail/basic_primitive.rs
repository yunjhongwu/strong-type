use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic_primitive(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn value(&self) -> #value_type {
                self.0
            }
        }

        impl From<#value_type> for #name {
            fn from(value: #value_type) -> Self {
                Self(value)
            }
        }

        impl Copy for #name {}

        impl Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }

        #[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
        impl std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                 self.value().partial_cmp(&rhs.value())
            }
        }
    }
}
