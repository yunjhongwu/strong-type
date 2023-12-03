use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic_string(name: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn value(&self) -> &str {
                self.0.as_str()
            }
        }

        impl From<&str> for #name {
            fn from(value: &str) -> Self {
                Self(String::from(value))
            }
        }

        impl From<String> for #name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                Some(self.value().cmp(&rhs.value()))
            }
        }
    }
}
