mod detail;
mod strong_type;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::strong_type::expand_strong_type;

#[proc_macro_derive(StrongType, attributes(numeric, custom_display))]
pub fn strong_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_strong_type(input).into()
}
