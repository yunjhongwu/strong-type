use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                 .field("value", &self.0)
                 .finish()
            }
        }

        impl std::default::Default for #name {
            fn default() -> Self {
                Self::new(#value_type::default())
            }
        }

        impl std::cmp::PartialEq for #name {
            fn eq(&self, rhs: &Self) -> bool {
                self.value() == rhs.value()
            }
        }

        unsafe impl Send for #name {}

        unsafe impl Sync for #name {}
    }
}