mod opt;
mod refs;
use opt::Opt;
use proc_macro as pm;
use proc_macro2 as pm2;
use quote::{format_ident, quote};
use refs::*;
use std::borrow::Borrow;
use syn::{
    parse_macro_input, GenericArgument, ItemFn, Meta, PathArguments, ReturnType, Signature, Type,
    TypePath,
};

fn get_result_type(sig: &Signature) -> Option<(&Type, &Type)> {
    let ReturnType::Type(_, return_type) = &sig.output else {
        return None;
    };
    let Type::Path(TypePath { path, .. }) = &**return_type else {
        return None;
    };
    if path.segments.len() != 1 || path.segments[0].ident != "Result" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &path.segments[0].arguments else {
        return None;
    };
    if args.args.len() != 2 {
        return None;
    }
    let (GenericArgument::Type(type_ok), GenericArgument::Type(type_err)) =
        (&args.args[0], &args.args[1])
    else {
        return None;
    };
    Some((type_ok, type_err))
}

fn get_docs(input: pm::TokenStream) -> Result<syn::LitStr, pm::TokenStream> {
    let input: syn::Item = syn::parse(input).unwrap();

    let attrs = match &input {
        syn::Item::Fn(inner) => &inner.attrs,
        syn::Item::Struct(inner) => &inner.attrs,
        syn::Item::Mod(inner) => &inner.attrs,
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "#[docs] is only supported on functions, structs or modules",
            )
            .to_compile_error()
            .into());
        }
    };

    let docs = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                match &attr.meta {
                    Meta::NameValue(nv) => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }) = &nv.value
                        {
                            Some(lit.value())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(syn::LitStr::new(&docs, proc_macro2::Span::call_site()))
}

#[proc_macro_attribute]
pub fn import(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let doc_literal = match get_docs(input.clone()) {
        Ok(docs) => docs,
        Err(err) => {
            return err;
        }
    };
    let opt: Opt = parse_macro_input!(args as Opt);
    let item_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &item_fn.sig.ident;
    let args = &item_fn.sig.inputs;
    let args_required: usize = args.len();
    let mut arguments: Vec<pm2::TokenStream> = Vec::new();
    let mut declarations: Vec<pm2::TokenStream> = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if let syn::FnArg::Typed(pat_type) = arg {
            let name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                let name = (&pat_ident.ident).clone();
                quote! {
                    Some(stringify!(#name).to_string())
                }
            } else {
                quote! {
                    None
                }
            };
            match pat_type.ty.borrow() {
                Type::Path(_ty) => {
                    arguments.push(quote! {
                        args[#i].take().unwrap().value.try_to_rs().map_err(|err| LinkedErr::by_link(err, (&caller).into()))?,
                    });
                    match get_ty(&pat_type.ty) {
                        Ok(ty) => {
                            declarations.push(quote! {
                                FnArgDesc {
                                    ty: #ty.into(),
                                    name: #name,
                                    docs: None,
                                }

                            });
                        }
                        Err(err) => {
                            return syn::Error::new_spanned(pat_type, err)
                                .to_compile_error()
                                .into();
                        }
                    };
                }
                _ => {
                    return syn::Error::new_spanned(pat_type, "Only Type::Path are supported")
                        .to_compile_error()
                        .into();
                }
            }
        } else {
            return syn::Error::new_spanned(arg, "Method receivers are not supported")
                .to_compile_error()
                .into();
        }
    }
    let Some((type_ok, _type_err)) = get_result_type(&item_fn.sig) else {
        return syn::Error::new_spanned(item_fn.sig, "Return type can be only Result<T,E>")
            .to_compile_error()
            .into();
    };
    let type_ok = match get_ty(type_ok) {
        Ok(ty) => ty,
        Err(err) => {
            return syn::Error::new_spanned(type_ok, err)
                .to_compile_error()
                .into();
        }
    };
    let func_name = format_ident!("{}_executor", fn_name);
    let reference = if opt.ns.is_empty() {
        fn_name.to_string()
    } else {
        format!("{}::{fn_name}", opt.ns)
    };

    pm::TokenStream::from(quote! {
        fn #func_name(args: Vec<FnArgValue>, _rt: Runtime, _cx: Context, caller: SrcLink) -> RtPinnedResult<'static, LinkedErr<E>> {
            Box::pin(async move {
                if args.len() != #args_required {
                    return Err(LinkedErr::by_link(E::InvalidFnArgument, (&caller).into()));
                }
                #item_fn;
                let mut args = args
                    .into_iter()
                    .map(Some)
                    .collect::<Vec<Option<FnArgValue>>>();
                let result = #fn_name(#(#arguments)*)
                    .map_err(|err| LinkedErr::by_link(err, (&caller).into()))?;
                result.try_to_rtv().map_err(|err| LinkedErr::by_link(err, (&caller).into()))
            })
        }

        efns.add(
            #reference.to_string(),
            EmbeddedFnEntity {
                uuid: Uuid::new_v4(),
                fullname: #reference.to_string(),
                name: stringify!(#fn_name).to_string(),
                docs: String::from(#doc_literal),
                args: vec![#(#declarations,)*],
                result: #type_ok,
                exec: #func_name,
            },
        )?;
    })
}

// let desc = ExecutorFnDescription::new(
//     #func_name,
//     vec![#(#declarations,)*],
//     #output_declaration
// );
// store.insert(
//     #reference,
//     desc,
// )?;
