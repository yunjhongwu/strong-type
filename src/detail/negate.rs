use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_negate(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Neg for #name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.value())
            }
        }
    }
}
