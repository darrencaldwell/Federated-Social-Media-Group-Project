use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, Block};

#[proc_macro_attribute]
pub fn protected(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);
    let vis = input.vis;
    let mut sig = input.sig;
    let req = (quote! {req: HttpRequest}).into();
    sig.inputs.push(parse_macro_input!(req as syn::FnArg));
    let block: Block = *input.block;
    (quote! { #vis #sig {
            if let Some(token) = req.headers().get("Authorization") {
                if let Ok(token) = token.to_str() {
                    if token.len() > 8 {
                        if let Ok(user_id) = decode_jwt(&token[7..]) {
                            return #block
                        }
                    }
                }
            }

            HttpResponse::Forbidden().body("Invalid token")
        }
    }).into()
}

#[proc_macro_attribute]
pub fn auth_user(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);
    let vis = input.vis;
    let block: Block = *input.block;
    let attr = parse_macro_input!(attr as syn::ExprField);
    let mut sig = input.sig;
    let req = (quote! {req: HttpRequest}).into();
    sig.inputs.push(parse_macro_input!(req as syn::FnArg));
    (quote! {
        #vis #sig {
            if let Some(token) = req.headers().get("Authorization") {
                if let Ok(token) = token.to_str() {
                    if token.len() > 8 {
                        if let Ok(user_id) = decode_jwt(&token[7..]) {
                            if user_id == #attr {
                                return #block
                            }
                        }
                    }
                }
            }

            HttpResponse::Forbidden().body("Invalid token")
        }
    }).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
