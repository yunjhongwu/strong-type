use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_add(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Add<Self> for #name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.value() + rhs.value())
            }
        }

        impl std::ops::Add<&Self> for #name {
            type Output = <Self as std::ops::Add<Self>>::Output;

            fn add(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() + rhs.value())
            }
        }

        impl<'a> std::ops::Add<#name> for &'a #name {
            type Output = #name;

            fn add(self, rhs: #name) -> Self::Output {
                #name::new(self.value() + rhs.value())
            }
        }

        impl<'a> std::ops::Add<&#name> for &'a #name {
            type Output = #name;

            fn add(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() + rhs.value())
            }
        }

        impl std::ops::AddAssign<Self> for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.value()
            }
        }

        impl std::ops::AddAssign<&Self> for #name {
            fn add_assign(&mut self, rhs: &Self) {
                self.0 += rhs.value()
            }
        }
    }
}

pub(crate) fn implement_sub(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Sub<Self> for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.value() - rhs.value())
            }
        }

        impl std::ops::Sub<&#name> for #name {
            type Output = Self;
            fn sub(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() - rhs.value())
            }
        }

        impl<'a> std::ops::Sub<#name> for &'a #name {
            type Output = #name;
            fn sub(self, rhs: #name) -> Self::Output {
                #name::new(self.value() - rhs.value())
            }
        }

        impl<'a> std::ops::Sub<&#name> for &'a #name {
            type Output = #name;
            fn sub(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() - rhs.value())
            }
        }


        impl std::ops::SubAssign<Self> for #name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.value()
            }
        }

        impl std::ops::SubAssign<&Self> for #name {
            fn sub_assign(&mut self, rhs: &Self) {
                self.0 -= rhs.value()
            }
        }
    }
}

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
    let add = implement_add(name);
    let sub = implement_sub(name);
    let mul = implement_mul(name);
    let div = implement_div(name);
    let rem = implement_rem(name);

    quote! {
        #add
        #sub
        #mul
        #div
        #rem
    }
}
