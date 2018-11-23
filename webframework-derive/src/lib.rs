#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
extern crate regex;
extern crate lazy_static;

use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{parse_macro_input, braced, ItemFn, Ident, Token, LitStr,
    Visibility, Meta, Lit, FnArg, Pat, ArgCaptured, PatIdent};
use syn::token::Brace;
use syn::punctuated::Punctuated;
use regex::Regex;

#[proc_macro_attribute]
pub fn controller(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ItemFn { vis, ident, block, attrs, decl, .. } = func;

    let params: Vec<_> = attrs.iter().flat_map(|attr| attr.parse_meta().ok())
        .flat_map(|attr| if let Meta::NameValue(val) = attr { Some(val) } else { None } )
        .filter(|attr| attr.ident.to_string() == "params")
        .flat_map(|attr| if let Lit::Str(litstr) = attr.lit { Some(litstr) } else { None })
        .map(|litstr| litstr.value())
        .flat_map(|val| val.split(',').map(str::trim).map(String::from).collect::<Vec<_>>())
        .collect();

    let inputs: Vec<_> = decl.inputs.iter().flat_map(|input| {
        if let FnArg::Captured(ArgCaptured { pat: Pat::Ident(PatIdent { ident, .. }), ty, ..}) = input {
            Some((ty, ident))
        } else {
            None
        }
    }).collect();

    let input_tokens: Vec<_> = inputs.iter()
        .filter(|(_ty, input)| params.iter().find(|&param| *param == input.to_string()).is_none())
        .map(|(ty, input)| {
            quote! {
                let #input: #ty = ::webframework::request::FromRequest::from(&req)?;
            }
        }).collect();

    let param_tokens: Vec<_> = params.into_iter()
        .map(|param| {
            let (ty, ident) = inputs.iter().find(|&(_ty, ident)| {
                ident.to_string() == param
            }).expect(&format!("Param '{}' not found in arguments.", param));
            quote! {
                let param = match params.get(&String::from(#param)) {
                    Some(p) => p,
                    None => Err(::webframework::request::RequestError::ParamNotFound(String::from(#param)))?,
                };
                let #ident: #ty = ::webframework::request::FromParameter::from(param)?;
            }
        }).collect();;

    TokenStream::from(quote!(
        #[derive(Debug, Clone)]
        #vis struct #ident;

        impl ::webframework::router::Router for #ident {
            fn handle(&self, mut req: ::webframework::request::Request,
                      path: Option<String>, params: ::std::collections::HashMap<String, String>)
                -> ::webframework::router::RouterResult {
                let result = ||{
                    #(#input_tokens)*;
                    #(#param_tokens)*;
                    #block
                };
                ::webframework::router::RouterResult::Handled(
                    Box::new(::futures::future::result(result()))
                )
            }
        }
    ))
}

#[proc_macro_attribute]
pub fn meta_controller(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ItemFn { vis, ident, block, .. } = func;

    TokenStream::from(quote!(
        #vis fn #ident(req: ::webframework::request::Request) -> ::webframework::router::RouterFuture {
            let result = ||{ #block };
            Box::new(::futures::future::result(result()))
        }
    ))
}

struct Routing {
    visibility: Visibility,
    name: Ident,
    routes: Punctuated<Route, Token![;]>,
}


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
    Single(Ident),
    Meta(Ident, Ident),
}

struct Route {
    restrictions: Vec<Ident>,
    path: Option<LitStr>,
    kind: InnerRouteKind,
}

impl Parse for Route {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![>>]) {
            input.parse::<Token![>>]>()?;
            let name: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;
            let controller: Ident = input.parse()?;
            return Ok( Route { restrictions: vec![], path: None,
                kind: InnerRouteKind::Meta(name, controller) } );
        }

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

            lazy_static::lazy_static! {
                static ref items: Regex = Regex::new(":(?P<name>[a-zA-Z]+)").unwrap();
            }

            let path_value = path.value();
            let capture_path = items.replace_all(&path_value, "(?P<$name>[^/]+)");

            quote! {
                {
                    ::lazy_static::lazy_static! {
                        static ref RE: ::regex::Regex = {
                            let path = format!(r"\A({})(?:/|\z)", #capture_path);
                            ::regex::Regex::new(&path).unwrap()
                        };
                    };

                    let matched_path = ::webframework::request_filters::PathFilter::handles(&*RE, &req, &path);

                    match matched_path {
                        ::webframework::request_filters::PathFilterResult::Matched(new_path, mut param) => {
                            path = new_path;
                            params.extend(param.drain());
                            true
                        }
                        ::webframework::request_filters::PathFilterResult::NotMatched => false,
                    }
                }
            }
        }).unwrap_or_else(|| quote!{ true });
        let handler = match &route.kind {
            InnerRouteKind::Multiple(mul) => {
                let inner = mul.iter().map(|inner| {
                    let restr = &inner.restrictions;
                    let ctrl = &inner.controller;
                    let assert_router = quote_spanned! {ctrl.span() =>
                        {
                            fn __assert_router<F: ::webframework::router::Router>(_: F) { }
                            __assert_router(#ctrl);
                        }
                    };
                    quote! {
                        if true #(&& ::webframework::request_filters::RequestFilter::handles(&#restr, &req))* {
                            #assert_router

                            match ::webframework::router::Router::handle(&#ctrl, req, Some(path.clone()), params) {
                                ::webframework::router::RouterResult::Handled(resp) => {
                                    return ::webframework::router::RouterResult::Handled(resp);
                                }
                                ::webframework::router::RouterResult::Unhandled(re, mut param) => {
                                    req = re;
                                    params = param;
                                }
                            }
                        }
                    }
                });
                quote! {
                    #(#inner);*
                }
            }
            InnerRouteKind::Single(sing) => {
                let assert_router = quote_spanned! {sing.span() =>
                    {
                        fn __assert_router<F: ::webframework::router::Router>(_: F) { }
                        __assert_router(#sing);
                    }
                };
                quote! {
                    #assert_router

                    match ::webframework::router::Router::handle(&#sing, req, Some(path.clone()), params) {
                        ::webframework::router::RouterResult::Handled(resp) => {
                            return ::webframework::router::RouterResult::Handled(resp);
                        }
                        ::webframework::router::RouterResult::Unhandled(re, mut param) => {
                            req = re;
                            params = param;
                        }
                    }
                }
            }
            InnerRouteKind::Meta(name, ctrl) => {
                let assert_meta = quote_spanned! {ctrl.span() =>
                    {
                        fn __assert_meta<F: ::webframework::router::MetaRouter>(_: F) { }
                        __assert_meta(#ctrl);
                    }
                };

                match &name.to_string()[..] {
                    "NotFound" => {
                        quote! {
                            #assert_meta

                            return ::webframework::router::RouterResult::Handled(
                                ::webframework::router::MetaRouter::handle(&#ctrl, req)
                            );
                        }
                    }
                    _ => {
                        panic!("Unknown meta element {}", name.to_string());
                    }
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

    let route_maps: Vec<_> = routes.iter().map(|_route| {
        quote! {

        }
    }).collect();

    let expanded = quote! {
        #[derive(Debug, Clone)]
        #visibility struct #name;

        impl ::webframework::router::Router for #name {
            fn handle(&self, mut req: ::webframework::request::Request,
                      path: Option<String>, mut params: ::std::collections::HashMap<String, String>)
                -> ::webframework::router::RouterResult {
                let mut path = path.unwrap_or_else(|| req.uri().path().to_string());
                #( #route_handlers );*;

                return ::webframework::router::RouterResult::Unhandled(req, params);
            }

            fn router_map(&self) -> Option<::webframework::router::RouterMap> {
                let mut map = ::webframework::router::RouterMap::new();

                #( #route_maps );*

                Some(map)
            }
        }
    };

    TokenStream::from(expanded)
}
