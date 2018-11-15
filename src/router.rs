use crate::request::Request;
use crate::response::Response;

use std::collections::HashMap;

use failure::Error;
use futures::Future;
use http;

pub type RouterFuture = Box<dyn Future<Item = Response, Error = Error> + Send>;

#[derive(Debug, Clone)]
pub struct Route {
    pub name: String,
    pub method: Option<http::Method>,
}

#[derive(Debug, Clone)]
pub enum RouteType {
    Route(Route),
    RouteMap(Box<RouterMap>),
}

pub type RouterMap = HashMap<String, RouteType>;

pub trait Router: Clone {
    fn handle(&self, req: Request) -> RouterFuture;
    fn router_map(&self) -> Option<RouterMap> {
        None
    }
}

impl<F: Clone> Router for F where F: Fn(Request) -> RouterFuture {
    fn handle(&self, req: Request) -> RouterFuture {
        self(req)
    }
}

macro_rules! routing {
    ( @all_true $($item:expr);* ) => {
        true $(&& $item)*
    };

    ( @inner_item $req:ident $ctrl:ident ) => {
        return $crate::router::Router::handle(&$ctrl, $req);
    };

    ( @respond_to $req:ident $($checks:ident)+ => $response:tt; $($tail:tt)* ) => {
        if routing!(@all_true $($checks(&$req));+) {
            routing!( @inner_item $req $response );
        }

        routing!( @respond_to $req $($tail)* );
    };

    ( @respond_to $req:ident ) => {
        ();
    };

    ( @item $req:ident $($checks:expr)+ => { $($response:tt)+ }; $($tail:tt)* ) => {
        if routing!(@all_true $($crate::request_filters::RequestFilter::handles(&$checks, &$req));+ ) {
            routing!( @respond_to $req $($response)+ );
        }
        routing!( @item $req $($tail)* );
    };

    ( @item $req:ident $($checks:expr)+ => $response:tt; $($tail:tt)* ) => {
        if routing!(@all_true $($crate::request_filters::RequestFilter::handles(&$checks, &$req));+ ) {
            let html = $crate::request_filters::html;
            routing!( @respond_to $req html => $response; );
        }
        routing!( @item $req $($tail)* );
    };

    ( @item $req:ident ) => {
        ();
    };

    ( @last_item $check:expr, $($checks:expr),+ ) => {
        routing!( @last_item $($checks),+ )
    };

    ( @last_item $check:expr ) => {
        $check
    };

    ( @router_item $map:ident $($checks:expr)+ => { $($response:tt)* }; $($tail:tt)* ) => {
        let path = $crate::request_filters::PathFilter::name(&routing!(@last_item $($checks),+));
        let route = $crate::router::Route {
            name: "Complex Route".to_string(),
            method: None,
        };

        $map.insert(path, $crate::router::RouteType::Route(route));
        routing!( @router_item $map $($tail)*);
    };

    ( @router_item $map:ident $($checks:expr)+ => $response:tt; $($tail:tt)* ) => {
        let path = $crate::request_filters::PathFilter::name(&routing!(@last_item $($checks),+));
        let route = $crate::router::Route {
            name: stringify!($response).to_string(),
            method: None,
        };

        $map.insert(path, $crate::router::RouteType::Route(route));
        routing!( @router_item $map $($tail)*);
    };

    ( @router_item $map:ident ) => { (); };

    ( $name:ident => { $($def:tt)* } ) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name;

        impl $crate::router::Router for $name {
            fn handle(&self, req: $crate::request::Request) -> $crate::router::RouterFuture {
                routing! ( @item req $($def)* );
                unreachable!();
            }

            fn router_map(&self) -> Option<$crate::router::RouterMap> {
                let mut map = $crate::router::RouterMap::new();

                routing!( @router_item map $($def)* );

                return Some(map);
            }
        }
    };
}
