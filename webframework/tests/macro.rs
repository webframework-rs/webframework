extern crate webframework as wfw;

use crate::wfw::prelude::*;

use std::collections::HashMap;

use hyper::{Request as HyperRequest, Body};
use slog;
use uuid;

#[controller]
fn handle_it(_req: &Request) -> WebResult<Response> {
    Ok(Response::from_string("Response it"))
}

routing! {
    SimpleRouter => {
        GET "/" => handle_it;
        GET "/bar/:test" => handle_it;
    }
}

routing! {
    ComplexRouter => {
        POST "/test" => {
            html => handle_it;
        };
    }
}

fn new_request(path: &str) -> Request {
    let req = HyperRequest::get(path).body(Body::empty()).unwrap();
    let logger = slog::Logger::root(slog::Discard, slog::o!());
    let id = uuid::Uuid::new_v4();

    Request::from_req(id, logger, req)
}

#[test]
fn check_router() {
    let _router = SimpleRouter;
    let _router = ComplexRouter;
}

#[test]
fn check_routing() {
    let router = SimpleRouter;

    let req = new_request("/");
    assert!(router.handle(req, None, HashMap::new()).is_handled());

    let req = new_request("/foo");
    assert!(router.handle(req, None, HashMap::new()).is_unhandled());

    let req = new_request("/bar/foo");
    assert!(router.handle(req, None, HashMap::new()).is_handled());

    let req = new_request("/bar/foo/nope");
    assert!(router.handle(req, None, HashMap::new()).is_unhandled());
}

#[controller]
#[params="test"]
fn dynamic(test: String)-> WebResult<Response> {
    assert_eq!(test, "foo");

    Ok(Response::from_string(""))
}

routing! {
    DynamicRouter => {
        GET "/bar/:test" => dynamic;
    }
}

#[test]
fn check_dynamic() {
    let router = DynamicRouter;

    let req = new_request("/bar/foo");
    assert!(router.handle(req, None, HashMap::new()).is_handled());
}
