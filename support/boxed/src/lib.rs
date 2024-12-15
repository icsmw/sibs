use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn boxed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    let original_block = function.block;

    function.block = syn::parse_quote!({
        Box::pin(async move #original_block)
    });

    let output = quote! { #function };
    output.into()
}
