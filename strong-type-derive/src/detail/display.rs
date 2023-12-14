use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_display(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!(#name), &self.0)
            }
        }
    }
}
