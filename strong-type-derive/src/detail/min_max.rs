use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_min_max(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            fn min() -> Self {
                Self(#value_type::MIN)
            }

            fn max() -> Self {
                Self(#value_type::MAX)
            }
        }
    }
}
