extern crate webframework as wfw;
extern crate futures;

use crate::wfw::prelude::*;

#[controller]
fn handle_it(_req: Request) -> WebResult<Response> {
    Ok(Response::from_string("Response it"))
}

routing! {
    SimpleRouter => {
        GET "/" => handle_it;
    }
}

routing! {
    ComplexRouter => {
        POST "/test" => {
            html => handle_it;
        };
    }
}

#[test]
fn check_router() {
    let _router = SimpleRouter;
    let _router = ComplexRouter;
}
