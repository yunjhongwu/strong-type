use crate::detail::underlying_type_utils::get_type_group;
use crate::detail::{TypeInfo, UnderlyingType, get_type};
use syn::{Data, DeriveInput, Fields};

pub(crate) struct StrongTypeAttributes {
    pub has_auto_operators: bool,
    pub has_addable: bool,
    pub has_scalable: bool,
    pub has_custom_display: bool,
    pub has_conversion: bool,
    pub type_info: TypeInfo,
}

pub(crate) fn get_attributes(input: &DeriveInput) -> Result<StrongTypeAttributes, syn::Error> {
    let mut attributes = StrongTypeAttributes {
        has_auto_operators: false,
        has_custom_display: false,
        has_conversion: false,
        has_addable: false,
        has_scalable: false,
        type_info: get_type(input)?,
    };

    for attr in input.attrs.iter() {
        if attr.path().is_ident("strong_type") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("auto_operators") {
                    attributes.has_auto_operators = true;
                    Ok(())
                } else if meta.path.is_ident("addable") {
                    attributes.has_addable = true;
                    Ok(())
                } else if meta.path.is_ident("scalable") {
                    attributes.has_scalable = true;
                    Ok(())
                } else if meta.path.is_ident("custom_display") {
                    attributes.has_custom_display = true;
                    Ok(())
                } else if meta.path.is_ident("conversion") {
                    attributes.has_conversion = true;
                    Ok(())
                } else if meta.path.is_ident("underlying") {
                    if let Ok(strm) = meta.value() {
                        if let Ok(primitive_type) = strm.parse::<syn::Ident>() {
                            attributes.type_info.type_group = get_type_group(&primitive_type, UnderlyingType::Derived);
                            attributes.type_info.primitive_type = primitive_type;
                        } else {
                            return Err(meta.error("Failed to parse custom underlying type. Expected a type identifier."));
                        }
                    }
                    Ok(())
                } else {
                    Err(meta.error("Invalid strong_type attribute. Valid attributes are: auto_operators, addable, scalable, custom_display, conversion, underlying=<type>"))
                }
            })?;
        }
    }
    Ok(attributes)
}

pub(crate) fn validate_struct(input: &DeriveInput) -> Result<(), syn::Error> {
    if let Data::Struct(data_struct) = &input.data
        && let Fields::Unnamed(fields_unnamed) = &data_struct.fields
        && fields_unnamed.unnamed.len() == 1
    {
        return Ok(());
    };
    Err(syn::Error::new_spanned(
        input,
        "StrongType can only be derived for tuple structs with exactly one field. Example: struct MyType(i32);",
    ))
}
