use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic(
    name: &syn::Ident,
    value_type: &syn::Ident,
    primitive_type: &syn::Ident,
) -> TokenStream {
    quote! {
        // Compile-time guarantee: strong type has the same size as its inner type
        const _: () = assert!(
            core::mem::size_of::<#name>() == core::mem::size_of::<#value_type>(),
        );

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

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!(#name))
                 .field("value", &self.0)
                 .finish()
            }
        }

        impl core::default::Default for #name
        where
            #value_type: core::default::Default,
        {
            fn default() -> Self {
                Self::new(#value_type::default())
            }
        }

        impl core::cmp::PartialEq for #name {
            fn eq(&self, rhs: &Self) -> bool {
                self.value() == rhs.value()
            }
        }

        impl core::convert::AsRef<#value_type> for #name {
            fn as_ref(&self) -> &#value_type {
                #name::as_ref(self)
            }
        }

        impl core::convert::AsMut<#value_type> for #name {
            fn as_mut(&mut self) -> &mut #value_type {
                #name::as_mut(self)
            }
        }
    }
}
