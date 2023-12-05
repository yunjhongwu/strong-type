use syn::{Data, DeriveInput, Type};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub(crate) enum UnderlyingTypeGroup {
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

pub(crate) fn get_type_group(value_type: &syn::Ident) -> UnderlyingTypeGroup {
    if value_type == "i8"
        || value_type == "i16"
        || value_type == "i32"
        || value_type == "i64"
        || value_type == "i128"
        || value_type == "isize"
    {
        return UnderlyingTypeGroup::Int;
    }
    if value_type == "u8"
        || value_type == "u16"
        || value_type == "u32"
        || value_type == "u64"
        || value_type == "u128"
        || value_type == "usize"
    {
        return UnderlyingTypeGroup::UInt;
    }
    if value_type == "f32" || value_type == "f64" {
        return UnderlyingTypeGroup::Float;
    }
    if value_type == "bool" {
        return UnderlyingTypeGroup::Bool;
    }
    if value_type == "char" {
        return UnderlyingTypeGroup::Char;
    }
    if value_type == "String" {
        return UnderlyingTypeGroup::String;
    }
    panic!("Unsupported type: {}", value_type);
}
