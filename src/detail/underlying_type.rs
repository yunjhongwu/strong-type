use syn::{Data, DeriveInput, Type};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub(crate) enum UnderlyingType {
    Int,
    Float,
    UInt,
    Bool,
    Char,
    String,
}

pub(crate) fn get_type_ident(input: &DeriveInput) -> &syn::Ident {
    if let Data::Struct(ref data_struct) = input.data {
        if let Type::Path(ref path) = &data_struct.fields.iter().next().unwrap().ty {
            return &path.path.segments.last().unwrap().ident;
        }
    }
    panic!("Unsupported input")
}

pub(crate) fn get_type(value_type: &syn::Ident) -> UnderlyingType {
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
