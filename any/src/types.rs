use quote::ToTokens;
use syn::{Error, Type};

pub fn get(ty: &Type) -> Result<String, Error> {
    if let Some(complex) = complext(ty)? {
        Ok(complex)
    } else {
        single(ty)
    }
}

fn single(ty: &Type) -> Result<String, Error> {
    if let Type::Path(tp) = ty {
        Ok(tp
            .path
            .segments
            .last()
            .ok_or(Error::new_spanned(ty, "Fail get single type"))?
            .ident
            .to_string())
    } else {
        Err(Error::new_spanned(ty, "Fail get single type"))
    }
}

fn complext(ty: &Type) -> Result<Option<String>, Error> {
    if let Type::Path(tp) = ty {
        if let Some(segment) = tp.path.segments.last() {
            if segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return Ok(Some(
                            checked(&format!("Vec<{}>", single(inner)?.to_token_stream()))
                                .ok_or(Error::new_spanned(inner, "Fail get type for Vec"))?,
                        ));
                    }
                }
                return Err(Error::new_spanned(tp, "Fail parse Vec type "));
            } else if segment.ident == "HashMap" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let (
                        Some(syn::GenericArgument::Type(left)),
                        Some(syn::GenericArgument::Type(right)),
                    ) = (args.args.first(), args.args.last())
                    {
                        return Ok(Some(
                            checked(&format!(
                                "HashMap<{}, {}>",
                                single(left)?.to_token_stream(),
                                single(right)?.to_token_stream()
                            ))
                            .ok_or(Error::new_spanned(left, "Fail get type for HashMap"))?,
                        ));
                    }
                }
                return Err(Error::new_spanned(tp, "Fail parse HashMap type "));
            }
        }
    }
    Ok(None)
}

fn checked(ty: &str) -> Option<String> {
    syn::parse_str::<Type>(ty)
        .map(|ty| Some(ty.to_token_stream().to_string()))
        .unwrap_or(None)
}
