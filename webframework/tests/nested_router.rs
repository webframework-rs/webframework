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
        GET "/bar/:test" => handle_it;
        GET "/" => handle_it;
    }
}

routing! {
    NestedRouter => {
        GET "/foo" => SimpleRouter;
    }
}

fn new_request(path: &str) -> Request {
    let req = HyperRequest::get(path).body(Body::empty()).unwrap();
    let logger = slog::Logger::root(slog::Discard, slog::o!());
    let id = uuid::Uuid::new_v4();

    Request::from_req(id, logger, req)
}

#[test]
fn nested_dynamic() {
    let router = NestedRouter;

    for path in &["/foo", "/foo/", "/foo/bar/blah"] {
        println!("Testing {}", path);
        let req = new_request(&path);
        assert!(router.handle(req, None, HashMap::new()).is_handled());
    }
}

