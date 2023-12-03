mod detail;
mod named_type;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::named_type::expand_named_type;

#[proc_macro_derive(NamedType, attributes(custom_display))]
pub fn named_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_named_type(input, false).into()
}

#[proc_macro_derive(NamedNumeric, attributes(custom_display))]
pub fn named_numeric(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_named_type(input, true).into()
}
