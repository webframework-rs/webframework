use crate::request::Request;
use crate::response::Response;

use failure::Error;
use futures::Future;

pub type RouterFuture = Box<dyn Future<Item = Response, Error = Error> + Send>;

pub trait Router: Clone {
    fn handle(&self, req: Request) -> RouterFuture;
}

impl<F: Clone> Router for F where F: Fn(Request) -> RouterFuture {
    fn handle(&self, req: Request) -> RouterFuture {
        self(req)
    }
}

#[macro_export]
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

    ( $name:ident => { $($def:tt)* } ) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name;

        impl $crate::router::Router for $name {
            fn handle(&self, req: $crate::request::Request) -> $crate::router::RouterFuture {
                routing! ( @item req $($def)* );
                unreachable!();
            }
        }
    };
}
