use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_constants(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub const ZERO: Self = Self(0 as #value_type);
            pub const ONE: Self = Self(1 as #value_type);
        }
    }
}

pub(crate) fn implement_constants_derived(
    name: &syn::Ident,
    value_type: &syn::Ident,
) -> TokenStream {
    quote! {
        impl #name {
            pub const ZERO: Self = Self(#value_type::ZERO);
            pub const ONE: Self = Self(#value_type::ONE);
        }
    }
}

pub(crate) fn implement_infinity(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub const INFINITY: Self = Self(#value_type::INFINITY);
            pub const NEG_INFINITY: Self = Self(#value_type::NEG_INFINITY);
        }
    }
}

pub(crate) fn implement_limit(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub const MIN: Self = Self(#value_type::MIN);
            pub const MAX: Self = Self(#value_type::MAX);
        }
    }
}
