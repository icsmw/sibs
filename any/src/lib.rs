mod allowed;
mod types;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Error, FnArg, ItemFn};

#[proc_macro_attribute]
pub fn check_type(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);
    let args = &item_fn.sig.inputs;

    if args.len() != 1 {
        return Error::new_spanned(&item_fn.sig.inputs, "Expecting only 1 argument")
            .to_compile_error()
            .into();
    };

    let arg_type = match &args[0] {
        FnArg::Typed(pat_type) => match types::get(&pat_type.ty) {
            Ok(ty) => ty,
            Err(err) => return err.into_compile_error().into(),
        },
        _ => {
            return Error::new_spanned(&args[0], "Method receivers are not supported")
                .to_compile_error()
                .into()
        }
    };

    if !allowed::is(&arg_type) {
        return Error::new_spanned(&args[0], format!("Not supported format: {arg_type}"))
            .to_compile_error()
            .into();
    }

    TokenStream::from(quote! {
        #item_fn
    })
}
