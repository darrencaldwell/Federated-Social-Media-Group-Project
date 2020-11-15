use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, Block};

#[proc_macro_attribute]
pub fn protected(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);
    let vis = input.vis;
    let mut sig = input.sig;
    let auth = (quote! {auth: BearerAuth}).into();
    sig.inputs.push(parse_macro_input!(auth as syn::FnArg));
    let block: Block = *input.block;
    (quote! {
        #vis #sig {
            if let Ok(user_id) = decode_jwt(auth.token()) {
                return #block
            }

            HttpResponse::Forbidden().body("Invalid token")
        }
    }).into()
}

#[proc_macro_attribute]
pub fn auth_user(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);
    let vis = input.vis;
    let sig = input.sig;
    let block: Block = *input.block;
    let attr = parse_macro_input!(attr as syn::ExprField);
    (quote! {
        #vis #sig {
            if let Ok(user_id) = decode_jwt(auth.token()) {
                if user_id == #attr {
                    return #block
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
