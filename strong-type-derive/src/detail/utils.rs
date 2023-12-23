use syn::{Data, DeriveInput, Fields, Visibility};

#[derive(Default)]
pub(crate) struct StrongTypeAttributes {
    pub has_auto_operators: bool,
    pub has_custom_display: bool,
}

pub(crate) fn get_attributes(input: &DeriveInput) -> StrongTypeAttributes {
    let mut attributes = StrongTypeAttributes::default();

    for attr in input.attrs.iter() {
        if attr.path().is_ident("strong_type") {
            if let Err(message) = attr.parse_nested_meta(|meta| {

                if meta.path.is_ident("auto_operators") {
                    attributes.has_auto_operators = true;
                    Ok(())
                } else if meta.path.is_ident("custom_display") {
                    attributes.has_custom_display = true;
                    Ok(())
                } else {
                    Err(meta.error(format!("Invalid strong_type attribute {}, should be one of {{auto_operators, custom_display}}",
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
