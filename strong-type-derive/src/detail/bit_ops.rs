use proc_macro2::{Span, TokenStream};
use quote::quote;

use super::macros::{impl_bitand, impl_bitor, impl_bitxor};

pub(crate) fn implement_bit_shift_impl(name: &syn::Ident, type_name_str: &str) -> TokenStream {
    let type_name = syn::Ident::new(type_name_str, Span::call_site());
    quote! {
        impl std::ops::Shl<#type_name> for #name {
            type Output = Self;

            fn shl(self, rhs: #type_name) -> Self::Output {
                Self::new(self.value() << rhs)
            }
        }

        impl std::ops::ShlAssign<#type_name> for #name {
            fn shl_assign(&mut self, rhs: #type_name) {
                self.0 <<= rhs;
            }
        }

        impl std::ops::Shr<#type_name> for #name {
            type Output = Self;

            fn shr(self, rhs: #type_name) -> Self::Output {
                Self::new(self.value() >> rhs)
            }
        }

        impl std::ops::ShrAssign<#type_name> for #name {
            fn shr_assign(&mut self, rhs: #type_name) {
                self.0 >>= rhs;
            }
        }

        impl std::ops::Shl<#type_name> for &#name {
            type Output = #name;

            fn shl(self, rhs: #type_name) -> Self::Output {
                 #name::new(self.value() << rhs)
            }
        }

        impl std::ops::Shr<#type_name> for &#name {
            type Output = #name;

            fn shr(self, rhs: #type_name) -> Self::Output {
                #name::new(self.value() >> rhs)
            }
        }
    }
}

pub(crate) fn implement_bit_shift(name: &syn::Ident) -> TokenStream {
    // Priority 2: Consolidate bit shift type loop
    const SHIFT_TYPES: &[&str] = &[
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ];

    let mut shift_impls = TokenStream::new();
    for shift_type in SHIFT_TYPES {
        shift_impls.extend(implement_bit_shift_impl(name, shift_type));
    }

    let bit_and_traits = impl_bitand(name);
    let bit_or_traits = impl_bitor(name);
    let bit_xor_traits = impl_bitxor(name);

    quote! {
        #shift_impls
        #bit_and_traits
        #bit_or_traits
        #bit_xor_traits
    }
}
