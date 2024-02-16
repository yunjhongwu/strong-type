use crate::detail::underlying_type_utils::get_type_group;
use crate::detail::{get_type, TypeInfo, UnderlyingType};
use syn::{Data, DeriveInput, Fields, Visibility};

pub(crate) struct StrongTypeAttributes {
    pub has_auto_operators: bool,
    pub has_custom_display: bool,
    pub type_info: TypeInfo,
}

pub(crate) fn get_attributes(input: &DeriveInput) -> StrongTypeAttributes {
    let mut attributes = StrongTypeAttributes {
        has_auto_operators: false,
        has_custom_display: false,
        type_info: get_type(input),
    };

    for attr in input.attrs.iter() {
        if attr.path().is_ident("strong_type") {
            if let Err(message) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("auto_operators") {
                    attributes.has_auto_operators = true;
                    Ok(())
                } else if meta.path.is_ident("custom_display") {
                    attributes.has_custom_display = true;
                    Ok(())
                } else if meta.path.is_ident("underlying") {
                    if let Ok(strm) = meta.value() {
                        if let Ok(primitive_type) = strm.parse::<syn::Ident>() {
                            attributes.type_info.type_group = get_type_group(&primitive_type, UnderlyingType::Derived);
                            attributes.type_info.primitive_type = primitive_type;
                        } else {
                            panic!("Failed to parse custom underlying {}", strm);
                        }
                    }
                    Ok(())
                } else {
                    Err(meta.error(format!("Invalid strong_type attribute {}, should be one of {{auto_operators, custom_display, underlying=type}}",
                                           meta.path.get_ident().expect("Failed to parse strong_type attributes."))))
                }
            }) {
                panic!("{}", message);
            }
        }
    }
    attributes
}

pub(crate) fn is_struct_valid(input: &DeriveInput) -> bool {
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            return (fields_unnamed.unnamed.len() == 1)
                && matches!(
                    fields_unnamed.unnamed.first().unwrap().vis,
                    Visibility::Inherited
                );
        }
    }
    false
}
