mod opt;
use opt::Opt;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Borrow;
use syn::{parse_macro_input, ItemFn, Type};

#[proc_macro_attribute]
pub fn import(args: TokenStream, input: TokenStream) -> TokenStream {
    let opt: Opt = parse_macro_input!(args as Opt);
    let item_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &item_fn.sig.ident;
    let args = &item_fn.sig.inputs;
    let args_required: usize = args.len();
    let mut argsuments: Vec<proc_macro2::TokenStream> = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if let syn::FnArg::Typed(pat_type) = arg {
            match pat_type.ty.borrow() {
                Type::Array(_) | Type::Path(_) => argsuments.push(quote! {
                    args[#i].try_to()?,
                }),
                _ => {
                    panic!("Only Type::Array and Type::Path are supported");
                }
            }
        } else {
            panic!("Method receivers are not supported");
        }
    }
    let func_name = format_ident!("{}_executor", fn_name);
    let reference = if opt.ns.is_empty() {
        fn_name.to_string()
    } else {
        format!("{}::{fn_name}", opt.ns)
    };
    TokenStream::from(quote! {
        fn #func_name(args: Vec<AnyValue>, cx: Context, sc: Scope) -> ExecutorPinnedResult {
            Box::pin(async move {
                if args.len() != #args_required {
                    return Err(E::InvalidArgumentsCount(#args_required.to_string(), args.len().to_string()));
                }
                #item_fn;
                Ok(AnyValue::new(#fn_name(#(#argsuments)*)?))
            })
        }
        store.insert(
            #reference,
            #func_name,
        )?;
    })
}
