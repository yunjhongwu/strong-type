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

        #[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
        impl std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                 self.value().partial_cmp(&rhs.value())
            }
        }

        unsafe impl Send for #name {}

        unsafe impl Sync for #name {}
    }
}
