use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Type};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
enum UnderlyingType {
    Int,
    Float,
    UInt,
    Bool,
    Char,
    String,
}

pub(super) fn expand_named_type(input: DeriveInput, impl_arithmetic: bool) -> TokenStream {
    let name = &input.ident;
    let value_type = get_type(&input);
    let group = get_type_group(value_type);
    let impl_display = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("default_display"));

    let mut output_ast = quote!();
    output_ast.extend(expand_basic(name, value_type));
    output_ast.extend(expand_partial_eq(name));

    if impl_display {
        output_ast.extend(quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}({})", stringify!(#name), &self.0)
                }
            }
        });
    };

    if let UnderlyingType::Int | UnderlyingType::UInt | UnderlyingType::Float = &group {
        output_ast.extend(expand_extreme_values(name, value_type));
    }

    if let UnderlyingType::Float = &group {
        output_ast.extend(expand_nan(name, value_type));
    }

    if let UnderlyingType::Bool = &group {
        output_ast.extend(quote! {
            impl std::ops::Not for #name {
               type Output = Self;

                fn not(self) -> Self::Output {
                    #name(!self.value())
                }
            }
        });
    }

    if impl_arithmetic {
        match &group {
            UnderlyingType::Int | UnderlyingType::UInt | UnderlyingType::Float => {
                output_ast.extend(expand_arithmetic(name));
                if let UnderlyingType::Int | UnderlyingType::Float = &group {
                    output_ast.extend(expand_signed_ops(name));
                }
            }
            _ => panic!("Non-arithmetic type {value_type}"),
        }
    }

    if let UnderlyingType::Int
    | UnderlyingType::UInt
    | UnderlyingType::Bool
    | UnderlyingType::Char
    | UnderlyingType::String = &group
    {
        output_ast.extend(expand_hashable(name));
    }

    output_ast
}

fn get_type(input: &DeriveInput) -> &syn::Ident {
    if let Data::Struct(ref data_struct) = input.data {
        if let Type::Path(ref path) = &data_struct.fields.iter().next().unwrap().ty {
            return &path.path.segments.last().unwrap().ident;
        }
    }
    panic!("Unsupported input")
}

fn get_type_group(value_type: &syn::Ident) -> UnderlyingType {
    match value_type.to_string().as_str() {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => UnderlyingType::Int,
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => UnderlyingType::UInt,
        "f32" | "f64" => UnderlyingType::Float,
        "bool" => UnderlyingType::Bool,
        "char" => UnderlyingType::Char,
        "String" => UnderlyingType::String,
        _ => panic!("Unsupported type"),
    }
}

fn expand_basic(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    let basic = match value_type.to_string().as_str() {
        "String" => quote! {
            impl #name {
                pub fn new(value : &str) -> Self {
                    Self(String::from(value))
                }

                pub fn value(&self) -> &str {
                    self.0.as_str()
                }
            }

            impl Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }

            impl std::cmp::PartialOrd for #name {
                fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.value().cmp(&rhs.value()))
                }
            }
        },
        _ => quote! {
            impl #name {
                pub fn new(value : #value_type) -> Self {
                    Self(value)
                }

                pub fn value(&self) -> #value_type {
                    self.0
                }
            }

            impl Copy for #name {}

            impl Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }

            #[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
            impl std::cmp::PartialOrd for #name {
                fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                     self.value().partial_cmp(&rhs.value())
                }
            }
        },
    };

    let mut output_ast = quote!();
    output_ast.extend(basic);
    output_ast.extend(quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                 .field("value", &self.0)
                 .finish()
            }
        }
    });

    output_ast
}

fn expand_arithmetic(name: &syn::Ident) -> TokenStream {
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

fn expand_extreme_values(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            fn min() -> Self {
                Self(#value_type::MIN)
            }

            fn max() -> Self {
                Self(#value_type::MAX)
            }
        }
    }
}

fn expand_nan(name: &syn::Ident, value_type: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            fn nan() -> Self {
                Self(#value_type::NAN)
            }
        }
    }
}

fn expand_partial_eq(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::cmp::PartialEq for #name {
            fn eq(&self, rhs: &Self) -> bool {
                self.value() == rhs.value()
            }
        }
    }
}

fn expand_signed_ops(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::ops::Neg for #name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.value())
            }
        }
    }
}

fn expand_hashable(name: &syn::Ident) -> TokenStream {
    quote! {
        impl std::cmp::Eq for #name {}

        impl std::cmp::Ord for #name {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                self.value().cmp(&rhs.value())
            }
        }

        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value().hash(state);
            }
       }
    }
}
