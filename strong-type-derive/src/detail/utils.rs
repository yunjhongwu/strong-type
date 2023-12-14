use syn::DeriveInput;

pub(crate) fn has_numeric(input: &DeriveInput) -> bool {
    input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("numeric"))
}

pub(crate) fn has_custom_display(input: &DeriveInput) -> bool {
    input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("custom_display"))
}
