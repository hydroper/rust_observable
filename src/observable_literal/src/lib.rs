#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Ident, Token};

struct Observer {
    map: HashMap<String, (Ident, Expr)>,
}

impl Parse for Observer {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut map = HashMap::new();
        while !input.is_empty() {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                break;
            }
            let id = input.parse::<Ident>()?;
            match id.to_string().as_str() {
                "start" | "next" | "error" | "complete" => {
                    input.parse::<Token![:]>()?;
                    if map.contains_key(&id.to_string()) {
                        id.span().unwrap().error(format!(r#"Duplicating "{}" method"#, id)).emit();
                    }
                    map.insert(id.to_string(), (id, input.parse::<Expr>()?));
                },
                _ => {
                    id.span().unwrap().error(format!(r#"Unknown "{}" method"#, id)).emit();
                },
            }
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Self { map })
    }
}

/// The `observer!` macro constructs an `Observer` by allowing
/// you to omit any of the listeners and not needing to box them explictly.
#[proc_macro]
pub fn observer(input: TokenStream) -> TokenStream {
    let Observer { map } = parse_macro_input!(input as Observer);
    let mut fields = proc_macro2::TokenStream::new();
    for name in ["next", "error", "start"] {
        fields.extend({
            // quote_spanned!(expr.span()=> #expr)
            let (id, callback) = map.get(name).map_or((Ident::new(name, Span::call_site()), quote!(|_| {})), |(id, expr)| (id.clone(), expr.into_token_stream()));
            quote!(#id: ::std::boxed::Box::new(#callback),)
        });
    }

    let complete_fn = map.get("complete").map_or(quote!(|| {}), |(_, expr)| expr.into_token_stream());

    quote!(
        Observer::<_, _> {
            #fields
            complete: ::std::boxed::Box::new(#complete_fn),
        }
    ).into()
}