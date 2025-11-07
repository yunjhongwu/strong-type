use syn::{Data, DeriveInput, Type};

pub(crate) enum UnderlyingType {
    Primitive,
    Derived,
}
pub(crate) enum ValueTypeGroup {
    Int(UnderlyingType),
    Float(UnderlyingType),
    UInt(UnderlyingType),
    Bool(UnderlyingType),
    Char(UnderlyingType),
    String(UnderlyingType),
}

pub(crate) struct TypeInfo {
    pub primitive_type: syn::Ident,
    pub value_type: syn::Ident,
    pub type_group: Option<ValueTypeGroup>,
}

pub(crate) fn get_type_group(
    value_type: &syn::Ident,
    underlying_type: UnderlyingType,
) -> Option<ValueTypeGroup> {
    if value_type == "i8"
        || value_type == "i16"
        || value_type == "i32"
        || value_type == "i64"
        || value_type == "i128"
        || value_type == "isize"
    {
        return Some(ValueTypeGroup::Int(underlying_type));
    }
    if value_type == "u8"
        || value_type == "u16"
        || value_type == "u32"
        || value_type == "u64"
        || value_type == "u128"
        || value_type == "usize"
    {
        return Some(ValueTypeGroup::UInt(underlying_type));
    }
    if value_type == "f32" || value_type == "f64" {
        return Some(ValueTypeGroup::Float(underlying_type));
    }
    if value_type == "bool" {
        return Some(ValueTypeGroup::Bool(underlying_type));
    }
    if value_type == "char" {
        return Some(ValueTypeGroup::Char(underlying_type));
    }
    if value_type == "String" {
        return Some(ValueTypeGroup::String(underlying_type));
    }

    None
}

fn get_type_ident(input: &DeriveInput) -> Option<syn::Ident> {
    if let Data::Struct(ref data_struct) = input.data {
        if let Type::Path(path) = &data_struct.fields.iter().next().unwrap().ty {
            return Some(path.path.segments.last().unwrap().ident.clone());
        }
    }
    None
}

pub(crate) fn get_type(input: &DeriveInput) -> TypeInfo {
    if let Some(value_type) = get_type_ident(input) {
        return TypeInfo {
            primitive_type: value_type.clone(),
            value_type: value_type.clone(),
            type_group: get_type_group(&value_type, UnderlyingType::Primitive),
        };
    }
    panic!("Unable to find underlying value type");
}
