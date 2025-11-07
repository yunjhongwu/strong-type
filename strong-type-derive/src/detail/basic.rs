use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic(
    name: &syn::Ident,
    value_type: &syn::Ident,
    primitive_type: &syn::Ident,
) -> TokenStream {
    quote! {
        impl #name {
            pub fn new(value: impl Into<#value_type>) -> Self {
                Self(value.into())
            }

            pub fn into_inner(self) -> #value_type {
                self.0
            }

            pub fn as_ref(&self) -> &#value_type {
                &self.0
            }

            pub fn as_mut(&mut self) -> &mut #value_type {
                &mut self.0
            }
        }

        impl ::strong_type::StrongType for #name {
            type UnderlyingType = #value_type;
            type PrimitiveType = #primitive_type;
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                 .field("value", &self.0)
                 .finish()
            }
        }

        impl std::default::Default for #name
        where
            #value_type: std::default::Default,
        {
            fn default() -> Self {
                Self::new(#value_type::default())
            }
        }

        impl std::cmp::PartialEq for #name {
            fn eq(&self, rhs: &Self) -> bool {
                self.value() == rhs.value()
            }
        }

        impl std::convert::AsRef<#value_type> for #name {
            fn as_ref(&self) -> &#value_type {
                #name::as_ref(self)
            }
        }

        impl std::convert::AsMut<#value_type> for #name {
            fn as_mut(&mut self) -> &mut #value_type {
                #name::as_mut(self)
            }
        }
    }
}
