use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_constants(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub const MIN: Self = Self(#value_type::MIN);
            pub const MAX: Self = Self(#value_type::MAX);
            pub const ZERO: Self = Self(0 as #value_type);
        }
    }
}
