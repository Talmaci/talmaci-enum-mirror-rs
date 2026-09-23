use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute,
    Data,
    DeriveInput,
    Error,
    Fields,
    Path,
    Result,
};

pub(super) fn expand(input: DeriveInput) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "EnumMirror does not support generic enums",
        ));
    }

    let target_enum = parse_target_enum(&input.attrs)?;
    let source_enum = &input.ident;

    let Data::Enum(data) = &input.data else {
        return Err(Error::new_spanned(
            source_enum,
            "EnumMirror can only be derived for enums",
        ));
    };

    let variants = data
        .variants
        .iter()
        .map(|variant| {
            if !matches!(variant.fields, Fields::Unit) {
                return Err(Error::new_spanned(
                    &variant.fields,
                    "EnumMirror supports unit variants only",
                ));
            }

            Ok(&variant.ident)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl ::core::convert::From<#source_enum> for #target_enum {
            fn from(value: #source_enum) -> Self {
                match value {
                    #(
                        #source_enum::#variants => Self::#variants,
                    )*
                }
            }
        }

        impl ::core::convert::From<#target_enum> for #source_enum {
            fn from(value: #target_enum) -> Self {
                match value {
                    #(
                        #target_enum::#variants => Self::#variants,
                    )*
                }
            }
        }
    })
}

fn parse_target_enum(attributes: &[Attribute]) -> Result<Path> {
    let attribute = attributes
        .iter()
        .find(|attribute| attribute.path().is_ident("enum_mirror"))
        .ok_or_else(|| {
            Error::new(
                proc_macro2::Span::call_site(),
                "EnumMirror requires #[enum_mirror(path::to::TargetEnum)]",
            )
        })?;

    attribute.parse_args::<Path>()
}