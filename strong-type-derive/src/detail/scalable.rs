use proc_macro2::TokenStream;
use quote::quote;

use super::macros::{impl_scalar_div, impl_scalar_mul, impl_scalar_rem};

pub(crate) fn implement_scalable(ident: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    let mul_ops = impl_scalar_mul(ident, value_type);
    let div_ops = impl_scalar_div(ident, value_type);
    let rem_ops = impl_scalar_rem(ident, value_type);

    quote! {
        #mul_ops
        #div_ops
        #rem_ops
    }
}
