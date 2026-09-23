mod enum_mirror;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(EnumMirror, attributes(enum_mirror))]
pub fn enum_mirror(input: TokenStream) -> TokenStream {
    enum_mirror::expand(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}