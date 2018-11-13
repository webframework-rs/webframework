#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;

use crate::proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ItemFn { attrs, vis, constness, unsafety, asyncness, abi, ident, decl, block, .. } = func;

    TokenStream::from(quote!(
        #vis fn #ident(req: ::webframework::request::Request)
            -> Box<dyn ::futures::Future<Item = ::webframework::response::Response, Error = ::failure::Error> + Send> {
            let result = ||{ #block };
            return Box::new(::futures::future::result(result()));
        }
    ))
}
