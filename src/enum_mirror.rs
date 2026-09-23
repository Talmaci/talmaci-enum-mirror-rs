use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Error, Fields, Path, Result};

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
    let mut attributes = attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("enum_mirror"));
    let attribute = attributes.next().ok_or_else(|| {
        Error::new(
            proc_macro2::Span::call_site(),
            "EnumMirror requires #[enum_mirror(path::to::TargetEnum)]",
        )
    })?;

    if let Some(duplicate) = attributes.next() {
        return Err(Error::new_spanned(
            duplicate,
            "EnumMirror expects exactly one #[enum_mirror(...)] attribute",
        ));
    }

    attribute.parse_args::<Path>()
}

#[cfg(test)]
mod tests {
    use super::expand;
    use syn::parse_quote;

    #[test]
    fn rejects_invalid_declarations() {
        let cases = [
            (
                parse_quote!(
                    enum Source {
                        Active,
                    }
                ),
                "requires #[enum_mirror",
            ),
            (
                parse_quote!(
                    #[enum_mirror(Target)]
                    struct Source;
                ),
                "only be derived for enums",
            ),
            (
                parse_quote!(#[enum_mirror(Target)] union Source { value: u8 }),
                "only be derived for enums",
            ),
            (
                parse_quote!(
                    #[enum_mirror(Target)]
                    enum Source<T> {
                        Value(T),
                    }
                ),
                "does not support generic enums",
            ),
            (
                parse_quote!(
                    #[enum_mirror(Target)]
                    enum Source {
                        Value(u8),
                    }
                ),
                "unit variants only",
            ),
            (
                parse_quote!(
                    #[enum_mirror(Target)]
                    enum Source {
                        Value { value: u8 },
                    }
                ),
                "unit variants only",
            ),
            (
                parse_quote!(
                    #[enum_mirror(Target)]
                    #[enum_mirror(Other)]
                    enum Source {
                        Active,
                    }
                ),
                "exactly one",
            ),
        ];
        for (input, expected) in cases {
            let error = expand(input).unwrap_err().to_string();
            assert!(
                error.contains(expected),
                "expected {expected:?}, got {error:?}"
            );
        }
    }

    #[test]
    fn rejects_malformed_target_attributes() {
        for input in [
            parse_quote!(
                #[enum_mirror]
                enum Source {
                    Active,
                }
            ),
            parse_quote!(
                #[enum_mirror()]
                enum Source {
                    Active,
                }
            ),
            parse_quote!(
                #[enum_mirror(Target, Other)]
                enum Source {
                    Active,
                }
            ),
            parse_quote!(
                #[enum_mirror = "Target"]
                enum Source {
                    Active,
                }
            ),
        ] {
            assert!(expand(input).is_err());
        }
    }
}
