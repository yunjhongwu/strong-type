use syn::DeriveInput;

pub(crate) fn has_auto_operators(input: &DeriveInput) -> bool {
    input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("auto_operators"))
}

pub(crate) fn has_custom_display(input: &DeriveInput) -> bool {
    input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("custom_display"))
}
