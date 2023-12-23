use proc_macro2::{Span, TokenStream};
use quote::quote;

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
    let traits_for_i8 = implement_bit_shift_impl(name, "i8");
    let traits_for_i16 = implement_bit_shift_impl(name, "i16");
    let traits_for_i32 = implement_bit_shift_impl(name, "i32");
    let traits_for_i64 = implement_bit_shift_impl(name, "i64");
    let traits_for_i128 = implement_bit_shift_impl(name, "i128");
    let traits_for_isize = implement_bit_shift_impl(name, "isize");
    let traits_for_u8 = implement_bit_shift_impl(name, "u8");
    let traits_for_u16 = implement_bit_shift_impl(name, "u16");
    let traits_for_u32 = implement_bit_shift_impl(name, "u32");
    let traits_for_u64 = implement_bit_shift_impl(name, "u64");
    let traits_for_u128 = implement_bit_shift_impl(name, "u128");
    let traits_for_usize = implement_bit_shift_impl(name, "usize");

    quote! {
        #traits_for_i8
        #traits_for_i16
        #traits_for_i32
        #traits_for_i64
        #traits_for_i128
        #traits_for_isize
        #traits_for_u8
        #traits_for_u16
        #traits_for_u32
        #traits_for_u64
        #traits_for_u128
        #traits_for_usize
    }
}
