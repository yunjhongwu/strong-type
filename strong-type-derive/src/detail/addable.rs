use proc_macro2::TokenStream;
use quote::quote;

use super::macros::{impl_add, impl_sub};

pub(crate) fn implement_addable(ident: &syn::Ident) -> TokenStream {
    let add_ops = impl_add(ident);
    let sub_ops = impl_sub(ident);

    quote! {
        #add_ops
        #sub_ops
    }
}
