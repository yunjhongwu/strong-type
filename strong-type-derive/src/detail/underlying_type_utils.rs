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
    pub type_group: ValueTypeGroup,
}

fn get_type_ident(input: &DeriveInput) -> Option<syn::Ident> {
    if let Data::Struct(ref data_struct) = input.data {
        if let Type::Path(ref path) = &data_struct.fields.iter().next().unwrap().ty {
            return Some(path.path.segments.last().unwrap().ident.clone());
        }
    }
    None
}

fn get_primitive_from_custom_underlying(input: &DeriveInput) -> Option<syn::Ident> {
    for attr in input.attrs.iter() {
        if attr.path().is_ident("custom_underlying") {
            let mut primitive = None;
            attr.parse_nested_meta(|meta| match meta.path.get_ident() {
                Some(ident) => {
                    primitive = Some(ident.clone());
                    Ok(())
                }
                None => Err(meta.error("Unsupported attribute")),
            })
            .ok()?;
            return primitive;
        }
    }

    None
}

pub(crate) fn get_type(input: &DeriveInput) -> TypeInfo {
    if let Some(value_type) = get_type_ident(input) {
        match get_primitive_from_custom_underlying(input) {
            Some(primitive_type) => TypeInfo {
                primitive_type: primitive_type.clone(),
                value_type: value_type.clone(),
                type_group: get_type_group(&primitive_type, UnderlyingType::Derived),
            },
            None => TypeInfo {
                primitive_type: value_type.clone(),
                value_type: value_type.clone(),
                type_group: get_type_group(&value_type, UnderlyingType::Primitive),
            },
        }
    } else {
        panic!("Unsupported input")
    }
}

pub(crate) fn get_type_group(
    value_type: &syn::Ident,
    underlying_type: UnderlyingType,
) -> ValueTypeGroup {
    if value_type == "i8"
        || value_type == "i16"
        || value_type == "i32"
        || value_type == "i64"
        || value_type == "i128"
        || value_type == "isize"
    {
        return ValueTypeGroup::Int(underlying_type);
    }
    if value_type == "u8"
        || value_type == "u16"
        || value_type == "u32"
        || value_type == "u64"
        || value_type == "u128"
        || value_type == "usize"
    {
        return ValueTypeGroup::UInt(underlying_type);
    }
    if value_type == "f32" || value_type == "f64" {
        return ValueTypeGroup::Float(underlying_type);
    }
    if value_type == "bool" {
        return ValueTypeGroup::Bool(underlying_type);
    }
    if value_type == "char" {
        return ValueTypeGroup::Char(underlying_type);
    }
    if value_type == "String" {
        return ValueTypeGroup::String(underlying_type);
    }
    panic!("Unsupported type: {}", value_type);
}
