use crate::detail::addable::implement_addable;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_mul(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Mul<Self> for #name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.value() * rhs.value())
            }
        }

        impl std::ops::Mul<&Self> for #name {
            type Output = Self;
            fn mul(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() * rhs.value())
            }
        }

        impl<'a> std::ops::Mul<#name> for &'a #name {
            type Output = #name;
            fn mul(self, rhs: #name) -> Self::Output {
                #name::new(self.value() * rhs.value())
            }
        }

        impl<'a> std::ops::Mul<&#name> for &'a #name {
            type Output = #name;
            fn mul(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() * rhs.value())
            }
        }

        impl std::ops::MulAssign<Self> for #name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.value()
            }
        }

        impl std::ops::MulAssign<&Self> for #name {
            fn mul_assign(&mut self, rhs: &Self) {
                self.0 *= rhs.value()
            }
        }

        impl std::iter::Product<Self> for #name {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, std::ops::Mul::mul)
            }
        }

        impl<'a> std::iter::Product<&'a Self> for #name {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, std::ops::Mul::mul)
            }
        }
    }
}

pub(crate) fn implement_div(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Div<Self> for #name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.value() / rhs.value())
            }
        }

        impl std::ops::Div<&Self> for #name {
            type Output = Self;
            fn div(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() / rhs.value())
            }
        }

        impl<'a> std::ops::Div<#name> for &'a #name {
            type Output = #name;
            fn div(self, rhs: #name) -> Self::Output {
                #name::new(self.value() / rhs.value())
            }
        }

        impl<'a> std::ops::Div<&#name> for &'a #name {
            type Output = #name;
            fn div(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() / rhs.value())
            }
        }

        impl std::ops::DivAssign<Self> for #name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.value()
            }
        }

        impl std::ops::DivAssign<&Self> for #name {
            fn div_assign(&mut self, rhs: &Self) {
                self.0 /= rhs.value()
            }
        }
    }
}

pub(crate) fn implement_rem(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Rem<Self> for #name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                Self::new(self.value() % rhs.value())
            }
        }

        impl std::ops::Rem<&Self> for #name {
            type Output = Self;
            fn rem(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() % rhs.value())
            }
        }

        impl<'a> std::ops::Rem<#name> for &'a #name {
            type Output = #name;
            fn rem(self, rhs: #name) -> Self::Output {
                #name::new(self.value() % rhs.value())
            }
        }

        impl<'a> std::ops::Rem<&#name> for &'a #name {
            type Output = #name;
            fn rem(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() % rhs.value())
            }
        }

        impl std::ops::RemAssign<Self> for #name {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.value()
            }
        }

        impl std::ops::RemAssign<&Self> for #name {
            fn rem_assign(&mut self, rhs: &Self) {
                self.0 %= rhs.value()
            }
        }
    }
}

pub(crate) fn implement_arithmetic(name: &syn::Ident) -> TokenStream {
    let addable = implement_addable(name);
    let mul = implement_mul(name);
    let div = implement_div(name);
    let rem = implement_rem(name);

    quote! {
        #addable
        #mul
        #div
        #rem
    }
}
