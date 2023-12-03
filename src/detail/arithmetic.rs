use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_arithmetic(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Add for #name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.value() + rhs.value())
            }
        }

        impl std::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.value()
            }
        }

        impl std::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.value() - rhs.value())
            }
        }

        impl std::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.value()
            }
        }

        impl std::ops::Mul for #name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.value() * rhs.value())
            }
        }

        impl std::ops::MulAssign for #name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.value()
            }
        }

        impl std::ops::Div for #name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self(self.value() / rhs.value())
            }
        }

        impl std::ops::DivAssign for #name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.value()
            }
        }
    }
}
