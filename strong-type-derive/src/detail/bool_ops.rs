use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_bool_ops(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Not for #name {
             type Output = Self;

             fn not(self) -> Self::Output {
                 Self::new(!self.value())
             }
        }

        impl std::ops::Not for &#name {
             type Output = #name;

             fn not(self) -> Self::Output {
                 #name::new(!self.value())
             }
        }

        impl std::ops::BitAnd<Self> for #name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::new(self.value() & rhs.value())
            }
        }

        impl std::ops::BitAnd<&Self> for #name {
            type Output = Self;

            fn bitand(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() & rhs.value())
            }
        }

        impl std::ops::BitAnd<#name> for &#name {
            type Output = #name;

            fn bitand(self, rhs: #name) -> Self::Output {
                #name::new(self.value() & rhs.value())
            }
        }

        impl std::ops::BitAnd<&#name> for &#name {
            type Output = #name;

            fn bitand(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() & rhs.value())
            }
        }

        impl std::ops::BitAndAssign<Self> for #name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.value()
            }
        }

        impl std::ops::BitAndAssign<&Self> for #name {
            fn bitand_assign(&mut self, rhs: &Self) {
                self.0 &= rhs.value()
            }
        }

        impl std::ops::BitOr<Self> for #name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::new(self.value() | rhs.value())
            }
        }

        impl std::ops::BitOr<&Self> for #name {
            type Output = Self;

            fn bitor(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() | rhs.value())
            }
        }

        impl std::ops::BitOr<#name> for &#name {
            type Output = #name;

            fn bitor(self, rhs: #name) -> Self::Output {
                #name::new(self.value() | rhs.value())
            }
        }

        impl std::ops::BitOr<&#name> for &#name {
            type Output = #name;

            fn bitor(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() | rhs.value())
            }
        }

        impl std::ops::BitOrAssign<Self> for #name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.value()
            }
        }

        impl std::ops::BitOrAssign<&Self> for #name {
            fn bitor_assign(&mut self, rhs: &Self) {
                self.0 |= rhs.value()
            }
        }

        impl std::ops::BitXor<Self> for #name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self::new(self.value() ^ rhs.value())
            }
        }

        impl std::ops::BitXor<&Self> for #name {
            type Output = Self;

            fn bitxor(self, rhs: &Self) -> Self::Output {
                Self::new(self.value() ^ rhs.value())
            }
        }

        impl std::ops::BitXor<#name> for &#name {
            type Output = #name;

            fn bitxor(self, rhs: #name) -> Self::Output {
                #name::new(self.value() ^ rhs.value())
            }
        }

        impl std::ops::BitXor<&#name> for &#name {
            type Output = #name;

            fn bitxor(self, rhs: &#name) -> Self::Output {
                #name::new(self.value() ^ rhs.value())
            }
        }

        impl std::ops::BitXorAssign<Self> for #name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.value()
            }
        }

        impl std::ops::BitXorAssign<&Self> for #name {
            fn bitxor_assign(&mut self, rhs: &Self) {
                self.0 ^= rhs.value()
            }
        }
    }
}
