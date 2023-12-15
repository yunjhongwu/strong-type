use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic_string(name: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn value(&self) -> &str {
                self.0.as_str()
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
    }
}
