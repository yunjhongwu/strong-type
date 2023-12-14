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
    }
}
