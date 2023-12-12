use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn implement_display(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!(#name), &self.0)
            }
        }
    }
}

pub(crate) fn custom_display(input: &DeriveInput) -> bool {
    input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("custom_display"))
}
