use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

fn get_value_ref_by_ident(ident: &Ident) -> TokenStream {
    match ident.to_string().as_str() {
        "u8" => {
            quote! { ValueRef::u8 }
        }
        "u16" => {
            quote! { ValueRef::u16 }
        }
        "u32" => {
            quote! { ValueRef::u32 }
        }
        "u64" => {
            quote! { ValueRef::u64 }
        }
        "u128" => {
            quote! { ValueRef::u128 }
        }
        "usize" => {
            quote! { ValueRef::usize }
        }
        "i8" => {
            quote! { ValueRef::i8 }
        }
        "i16" => {
            quote! { ValueRef::i16 }
        }
        "i32" => {
            quote! { ValueRef::i32 }
        }
        "i64" => {
            quote! { ValueRef::i64 }
        }
        "i128" => {
            quote! { ValueRef::i128 }
        }
        "isize" => {
            quote! { ValueRef::isize }
        }
        "bool" => {
            quote! { ValueRef::bool }
        }
        "String" => {
            quote! { ValueRef::String }
        }
        "PathBuf" => {
            quote! { ValueRef::PathBuf }
        }
        _ => {
            panic!("Not supported argument type")
        }
    }
}

pub fn get_value_ref(ty: &Type) -> TokenStream {
    match ty {
        Type::Path(ty) => {
            if let Some(ident) = ty.path.get_ident() {
                get_value_ref_by_ident(ident)
            } else if let Some(segment) = &ty.path.segments.first() {
                match segment.ident.to_string().as_str() {
                    "Vec" => {
                        let PathArguments::AngleBracketed(ref args) = segment.arguments else {
                            panic!("Only AngleBracketed is supported for Vec");
                        };
                        let Some(GenericArgument::Type(Type::Path(inner_ty))) = args.args.first()
                        else {
                            panic!("Only Path is supported for Vec");
                        };
                        let Some(inner_ident) = inner_ty.path.get_ident() else {
                            panic!("Fail to get ident for inner type of Vec");
                        };
                        let inner_ref = get_value_ref_by_ident(inner_ident);
                        quote! {
                            ValueRef::Vec(Box::new(#inner_ref))
                        }
                    }
                    "HashMap" => {
                        todo!("Not implemented");
                    }
                    "Option" => {
                        todo!("Not implemented");
                    }
                    "Result" => {
                        todo!("Not implemented");
                    }
                    _ => {
                        panic!("Only Vec is supported");
                    }
                }
            } else {
                panic!("Only Type::Path are supported");
            }
        }
        Type::Tuple(tuple) => {
            if tuple.elems.is_empty() {
                quote! { ValueRef::Empty }
            } else {
                panic!("Non-empty tuples arn't supported");
            }
        }
        _ => {
            panic!("Only Type::Path are supported");
        }
    }
}
