use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_nan(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            fn nan() -> Self {
                Self(#value_type::NAN)
            }

            fn is_nan(&self) -> bool {
                self.0.is_nan()
            }
        }
    }
}
