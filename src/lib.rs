extern crate webframework_derive;
#[macro_use] extern crate failure;
extern crate hyper;

pub mod prelude;
pub mod request;
pub mod response;
pub mod request_filters;
pub mod server;
pub mod router;
pub mod error;

pub use webframework_derive::controller;
#[macro_export]
macro_rules! routing {
    ( @all_true $($item:expr);* ) => {
        true $(&& $item)*
    };

    ( @inner_item $req:ident $ctrl:ident ) => {
        return $crate::router::Router::handle(&$ctrl, $req);
    };

    ( @respond_to $req:ident $($checks:ident)* => $response:tt; $($tail:tt)* ) => {
        if routing!(@all_true $($checks(&$req));+) {
            routing!( @inner_item $req $response );
        }

        routing!( @respond_to $req $($tail)* );
    };

    ( @respond_to $req:ident ) => {
        ();
    };

    ( @item $req:ident $($checks:expr)* => { $($response:tt)+ }; $($tail:tt)* ) => {
        if routing!(@all_true $($crate::request_filters::RequestFilter::handles(&$checks, &$req));+ ) {
            routing!( @respond_to $req $($response)+ );
        }
        routing!( @item $req $($tail)* );
    };

    ( @item $req:ident $($checks:expr)* => $response:tt; $($tail:tt)* ) => {
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
            fn handle(&self, req: $crate::request::Request)
                ->Box<dyn ::futures::Future<Item = $crate::response::Response, Error = ::failure::Error> + Send>{
                routing! ( @item req $($def)* );
                unreachable!();
            }
        }
    };
}

pub type WebResult<T> = ::std::result::Result<T, failure::Error>;
