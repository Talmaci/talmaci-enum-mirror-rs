#![doc = include_str!("../README.md")]

mod enum_mirror;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Generates consuming `From` implementations in both directions.
///
/// Specify exactly one `#[enum_mirror(path::to::Target)]` attribute.
/// Both enums must have matching unit variants. Derive on only one of them.
///
/// A target with additional variants cannot be converted exhaustively:
///
/// ```compile_fail
/// use talmaci_enum_mirror::EnumMirror;
/// #[derive(EnumMirror)]
/// #[enum_mirror(Target)]
/// enum Source { Active }
/// enum Target { Active, Disabled }
/// ```
///
/// A target with missing variants also fails to compile:
///
/// ```compile_fail
/// use talmaci_enum_mirror::EnumMirror;
/// #[derive(EnumMirror)]
/// #[enum_mirror(Target)]
/// enum Source { Active, Disabled }
/// enum Target { Active }
/// ```
///
/// Target variants must not carry data:
///
/// ```compile_fail
/// use talmaci_enum_mirror::EnumMirror;
/// #[derive(EnumMirror)]
/// #[enum_mirror(Target)]
/// enum Source { Active }
/// enum Target { Active(u8) }
/// ```
#[proc_macro_derive(EnumMirror, attributes(enum_mirror))]
pub fn enum_mirror(input: TokenStream) -> TokenStream {
    enum_mirror::expand(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
