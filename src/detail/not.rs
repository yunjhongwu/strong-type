use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_not(name: &syn::Ident) -> TokenStream {
    quote! {
    impl std::ops::Not for #name {
                   type Output = Self;

                    fn not(self) -> Self::Output {
                        #name(!self.value())
                    }
                }
        }
}
