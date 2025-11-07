use crate::detail::underlying_type_utils::get_type_group;
use crate::detail::{TypeInfo, UnderlyingType, ValueTypeGroup, get_type};
use syn::{Data, DeriveInput, Fields};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AutoOperatorMode {
    None,
    Full,
    Minimal,
}

pub(crate) struct StrongTypeAttributes {
    pub auto_operator_mode: AutoOperatorMode,
    pub has_addable: bool,
    pub has_scalable: bool,
    pub has_custom_display: bool,
    pub has_conversion: bool,
    pub type_info: TypeInfo,
}

/// Cached metadata about a strong type, resolved once during macro expansion.
/// This eliminates redundant type resolution and provides a clean data structure
/// to pass through code generation functions.
pub(crate) struct TypeMetadata {
    /// The name of the strong type being generated
    pub name: syn::Ident,
    /// The wrapped value type (e.g., i32, String, CustomType)
    pub value_type: syn::Ident,
    /// The primitive type (for derived types, this is the underlying primitive)
    pub primitive_type: syn::Ident,
    /// The type category (Int, UInt, Float, Bool, Char, String)
    pub type_group: ValueTypeGroup,
    /// How operators should be generated
    pub auto_operator_mode: AutoOperatorMode,
    /// Whether to generate addable operators (Add/Sub)
    pub has_addable: bool,
    /// Whether to generate scalable operators (scalar multiplication/division)
    pub has_scalable: bool,
    /// Whether the user provided a custom Display implementation
    pub has_custom_display: bool,
    /// Whether to generate From/Into conversion traits
    pub has_conversion: bool,
}

impl TypeMetadata {
    /// Creates a new TypeMetadata by parsing and resolving all type information once.
    pub fn new(input: &DeriveInput) -> Result<Self, syn::Error> {
        let attributes = get_attributes(input)?;

        let type_group = attributes.type_info.type_group.ok_or_else(|| {
            syn::Error::new_spanned(
                input,
                format!(
                    "Unable to determine the primitive type of '{}'. Supported types are: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char, String",
                    attributes.type_info.value_type
                ),
            )
        })?;

        Ok(Self {
            name: input.ident.clone(),
            value_type: attributes.type_info.value_type,
            primitive_type: attributes.type_info.primitive_type,
            type_group,
            auto_operator_mode: attributes.auto_operator_mode,
            has_addable: attributes.has_addable,
            has_scalable: attributes.has_scalable,
            has_custom_display: attributes.has_custom_display,
            has_conversion: attributes.has_conversion,
        })
    }
}

pub(crate) fn get_attributes(input: &DeriveInput) -> Result<StrongTypeAttributes, syn::Error> {
    let mut attributes = StrongTypeAttributes {
        auto_operator_mode: AutoOperatorMode::None,
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
                    // Check if there's a value assignment
                    if meta.input.peek(syn::Token![=]) {
                        let _: syn::Token![=] = meta.input.parse()?;
                        let value: syn::LitStr = meta.input.parse()?;
                        match value.value().as_str() {
                            "minimal" => attributes.auto_operator_mode = AutoOperatorMode::Minimal,
                            "full" => attributes.auto_operator_mode = AutoOperatorMode::Full,
                            other => return Err(meta.error(format!(
                                "Invalid auto_operators value '{}'. Valid values are: 'minimal', 'full'",
                                other
                            ))),
                        }
                    } else {
                        // No value means full mode (backward compatible)
                        attributes.auto_operator_mode = AutoOperatorMode::Full;
                    }
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
