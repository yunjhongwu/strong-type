use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_hash(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::cmp::Eq for #name {}

        impl std::cmp::Ord for #name {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                self.value().cmp(&rhs.value())
            }
        }

        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value().hash(state);
            }
       }
    }
}
