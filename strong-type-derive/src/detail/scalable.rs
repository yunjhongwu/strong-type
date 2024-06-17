use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_scalable(ident: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    let mul_ops = implement_mul(ident, value_type);
    let div_ops = implement_div(ident, value_type);
    let rem_ops = implement_rem(ident, value_type);

    quote! {
        #mul_ops
        #div_ops
        #rem_ops
    }
}

fn implement_mul(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Mul<#value_type> for #name {
            type Output = Self;
            fn mul(self, rhs: #value_type) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl<'a> std::ops::Mul<#value_type> for &'a #name {
            type Output = #name;
            fn mul(self, rhs: #value_type) -> Self::Output {
                #name(self.0 * rhs)
            }
        }

        impl std::ops::Mul<#name> for #value_type {
            type Output = #name;
            fn mul(self, rhs: #name) -> Self::Output {
                #name(self * rhs.0)
            }
        }

        impl<'a> std::ops::Mul<&#name> for #value_type {
            type Output = #name;
            fn mul(self, rhs: &#name) -> Self::Output {
                #name(self * rhs.0)
            }
        }

        impl std::ops::MulAssign<#value_type> for #name {
            fn mul_assign(&mut self, rhs: #value_type) {
                self.0 *= rhs;
            }
        }
    }
}

fn implement_div(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Div<#value_type> for #name {
            type Output = Self;
            fn div(self, rhs: #value_type) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl<'a> std::ops::Div<#value_type> for &'a #name {
            type Output = #name;
            fn div(self, rhs: #value_type) -> Self::Output {
                #name(self.0 / rhs)
            }
        }

        impl std::ops::DivAssign<#value_type> for #name {
            fn div_assign(&mut self, rhs: #value_type) {
                self.0 /= rhs;
            }
        }
    }
}

fn implement_rem(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Rem<#value_type> for #name {
            type Output = Self;
            fn rem(self, rhs: #value_type) -> Self::Output {
                Self(self.0 % rhs)
            }
        }

        impl<'a> std::ops::Rem<#value_type> for &'a #name {
            type Output = #name;
            fn rem(self, rhs: #value_type) -> Self::Output {
                #name(self.0 % rhs)
            }
        }

        impl std::ops::RemAssign<#value_type> for #name {
            fn rem_assign(&mut self, rhs: #value_type) {
                self.0 %= rhs;
            }
        }
    }
}
