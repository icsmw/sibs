use proc_macro::TokenStream;
use quote::quote;
use syn::Meta;

#[proc_macro_attribute]
pub fn docs(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::Item = syn::parse(item.clone()).unwrap();

    let attrs = match &input {
        syn::Item::Fn(inner) => &inner.attrs,
        syn::Item::Struct(inner) => &inner.attrs,
        syn::Item::Mod(inner) => &inner.attrs,
        _ => {
            return syn::Error::new_spanned(
                &input,
                "#[docs] is only supported on functions, structs or modules",
            )
            .to_compile_error()
            .into();
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

    let doc_literal = syn::LitStr::new(&docs, proc_macro2::Span::call_site());

    let output = quote! {
        pub const DOCS: &str = #doc_literal;
        #input
    };

    output.into()
}
