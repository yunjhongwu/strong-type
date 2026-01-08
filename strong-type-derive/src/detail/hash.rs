use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_hash(name: &syn::Ident) -> TokenStream {
    quote! {
        impl core::cmp::Eq for #name {}

        impl core::cmp::Ord for #name {
            fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
                self.value().cmp(&rhs.value())
            }
        }

        impl core::hash::Hash for #name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.value().hash(state);
            }
       }
    }
}
