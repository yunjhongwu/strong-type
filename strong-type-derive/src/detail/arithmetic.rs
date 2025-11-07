use crate::detail::addable::implement_addable;
use proc_macro2::TokenStream;
use quote::quote;

use super::macros::{impl_div, impl_mul, impl_rem};

pub(crate) fn implement_arithmetic(name: &syn::Ident) -> TokenStream {
    let addable = implement_addable(name);
    let mul = impl_mul(name);
    let div = impl_div(name);
    let rem = impl_rem(name);

    quote! {
        #addable
        #mul
        #div
        #rem
    }
}
