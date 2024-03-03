use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

#[derive(Clone, Debug, Default)]
pub struct Opt {
    pub ns: String,
}

impl Opt {
    pub(self) fn new(ns: String) -> Self {
        Self { ns }
    }
}

impl Parse for Opt {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        for expr in Punctuated::<Expr, Token![,]>::parse_terminated(input)?.into_iter() {
            match expr {
                Expr::Path(p) => {
                    if let Some(ident) = p.path.get_ident() {
                        return Ok(Opt::new(ident.to_string()));
                    } else {
                        return Err(syn::Error::new_spanned(p, "Cannot extract identification"));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "Expecting expr like [key = \"value as String\"] or [key]",
                    ));
                }
            }
        }
        Ok(Opt::new(String::new()))
    }
}
