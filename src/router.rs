use crate::request::Request;
use crate::response::Response;

use std::collections::HashMap;

use failure::Error;
use futures::Future;
use http;

pub type RouterFuture = Box<dyn Future<Item = Response, Error = Error> + Send>;

#[derive(Debug, Clone)]
pub struct RouteDetail {
    pub filters: Vec<String>,
    pub method: Option<http::Method>,
    pub specialisation: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Route {
    Route(RouteDetail),
    Map(Box<RouterMap>),
}

pub type RouterMap = HashMap<String, Route>;

pub enum RouterResult {
    Handled(RouterFuture),
    Unhandled(Request)
}

pub trait Router: Clone {
    fn handle(&self, req: Request, path: Option<String>) -> RouterResult;
    /// Returns a tree of routes by filters
    fn router_map(&self) -> Option<RouterMap> {
        None
    }
}

impl<F: Clone> Router for F where F: Fn(Request) -> RouterResult {
    fn handle(&self, req: Request, _path: Option<String>) -> RouterResult {
        self(req)
    }
}

pub trait MetaRouter {
    fn handle(&self, req: Request) -> RouterFuture;
}

impl<F: Clone> MetaRouter for F where F: Fn(Request) -> RouterFuture {
    fn handle(&self, req: Request) -> RouterFuture {
        self(req)
    }
}
