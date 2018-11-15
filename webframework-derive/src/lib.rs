#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{parse_macro_input, braced, ItemFn, Ident, Token, LitStr, Visibility};
use syn::token::Brace;
use syn::punctuated::Punctuated;

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

struct Routing {
    visibility: Visibility,
    name: Ident,
    routes: Punctuated<Route, Token![;]>,
}


/// This parses things like:
/// ```
/// routing! {
///     TaskRouter => {
///         POST "/create" => create_task;
///         GET "/" => {
///             html => tasks;
///             json => tasks_json;
///         };
///     }
/// }
/// ```
impl Parse for Routing {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let content;
        braced!(content in input);
        let routes: Punctuated<Route, Token![;]> = content.parse_terminated(Route::parse)?;
        Ok(Routing { visibility, name, routes })
    }
}

struct InnerRoute {
    restrictions: Vec<Ident>,
    controller: Ident,
}

enum InnerRouteKind {
    Multiple(Punctuated<InnerRoute, Token![;]>),
    Single(Ident)
}

struct Route {
    restrictions: Vec<Ident>,
    path: Option<LitStr>,
    kind: InnerRouteKind,
}

impl Parse for Route {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut restrictions: Vec<Ident> = vec![];
        while input.peek(Ident) {
            restrictions.push(input.parse()?);
        }
        let path: Option<LitStr> = input.parse().ok();
        input.parse::<Token![=>]>()?;
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            let inner_item = |input: ParseStream| {
                let mut restrictions : Vec<Ident> = vec![];
                while input.peek(Ident) {
                    restrictions.push(input.parse()?);
                }
                if restrictions.is_empty() {
                    return Err(input.error("you need to specify at least one filter"));
                }
                input.parse::<Token![=>]>()?;
                let controller: Ident = input.parse()?;
                Ok(InnerRoute { restrictions, controller })
            };

            let inner_items: Punctuated<InnerRoute, Token![;]> = content.parse_terminated(inner_item)?;
            return Ok(Route { restrictions, path, kind: InnerRouteKind::Multiple(inner_items) });
        } else {
            let inner_item: Ident = input.parse()?;
            return Ok(Route { restrictions, path, kind: InnerRouteKind::Single(inner_item) });
        }
    }
}

#[proc_macro]
pub fn routing(input: TokenStream) -> TokenStream {
    let Routing { visibility, name, routes } = parse_macro_input!(input as Routing);

    let route_handlers: Vec<_> = routes.iter().map(|route| {
        let restr = &route.restrictions;
        let path = route.path.as_ref().map(|path| {
            quote! {
                ::webframework::request_filters::RequestFilter::handles(&#path, &req)
            }
        }).unwrap_or_else(|| quote!{ true });
        let handler = match &route.kind {
            InnerRouteKind::Multiple(mul) => {
                let inner = mul.iter().map(|inner| {
                    let restr = &inner.restrictions;
                    let ctrl = &inner.controller;
                    quote! {
                        if true #(&& ::webframework::request_filters::RequestFilter::handles(&#restr, &req))* {
                            return ::webframework::router::Router::handle(&#ctrl, req);
                        }
                    }
                });
                quote! {
                    #(#inner);*
                }
            }
            InnerRouteKind::Single(sing) => {
                quote! {
                    return ::webframework::router::Router::handle(&#sing, req);
                }
            }
        };
        quote! {
            if true #(&& ::webframework::request_filters::RequestFilter::handles(&#restr, &req))* {
                if #path {
                    #handler;
                }
            }
        }
    }).collect();

    let expanded = quote! {
        #[derive(Debug, Clone)]
        #visibility struct #name;

        impl ::webframework::router::Router for #name {
            fn handle(&self, req: ::webframework::request::Request)
                -> ::webframework::router::RouterFuture {
                #( #route_handlers );*;
                unreachable!()
            }
        }
    };

    TokenStream::from(expanded)
}
