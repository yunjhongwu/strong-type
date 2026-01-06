use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_display(name: &syn::Ident) -> TokenStream {
    quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}({})", stringify!(#name), &self.0)
            }
        }
    }
}
