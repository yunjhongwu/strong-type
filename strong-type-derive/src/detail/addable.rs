use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_addable(ident: &syn::Ident) -> TokenStream {
    let add_ops = implement_add(ident);
    let sub_ops = implement_sub(ident);

    quote! {
        #add_ops
        #sub_ops
    }
}

fn implement_add(name: &syn::Ident) -> TokenStream {
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

        impl std::iter::Sum<Self> for #name {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, std::ops::Add::add)
            }
        }

        impl<'a> std::iter::Sum<&'a Self> for #name {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, std::ops::Add::add)
            }
        }
    }
}

fn implement_sub(name: &syn::Ident) -> TokenStream {
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
