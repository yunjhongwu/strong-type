use proc_macro2::TokenStream;
use quote::quote;

use super::macros::{impl_bitand, impl_bitor, impl_bitxor};

pub(crate) fn implement_bool_ops(name: &syn::Ident) -> TokenStream {
    let not_impl = quote! {
        impl std::ops::Not for #name {
             type Output = Self;

             fn not(self) -> Self::Output {
                 Self::new(!self.value())
             }
        }

        impl std::ops::Not for &#name {
             type Output = #name;

             fn not(self) -> Self::Output {
                 #name::new(!self.value())
             }
        }
    };

    let bit_and_traits = impl_bitand(name);
    let bit_or_traits = impl_bitor(name);
    let bit_xor_traits = impl_bitxor(name);

    quote! {
        #not_impl
        #bit_and_traits
        #bit_or_traits
        #bit_xor_traits
    }
}
