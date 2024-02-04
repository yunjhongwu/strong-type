use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic_string(name: &syn::Ident) -> TokenStream {
    quote! {
        impl Clone for #name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                 Some(self.cmp(rhs))
            }
        }
    }
}

pub(crate) fn implement_primitive_str_accessor(name: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn value(&self) -> &str {
                self.0.as_str()
            }

            pub fn primitive(&self) -> &str {
                self.value()
            }
        }
    }
}

pub(crate) fn implement_primitive_str_accessor_derived(
    name: &syn::Ident,
    value_type: &syn::Ident,
) -> TokenStream {
    quote! {
        impl #name {
            pub fn value(&self) -> &#value_type {
                &self.0
            }

            pub fn primitive(&self) -> &str {
                self.0.primitive()
            }
        }
    }
}
