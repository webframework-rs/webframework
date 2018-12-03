use crate::response::Response;
use crate::request::Request;

use std::collections::HashMap;

use failure::{Error, Context, Compat};
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
    Unhandled(Request, HashMap<String, String>)
}

pub trait Router: Clone {
    fn handle(&self, req: Request, path: Option<String>, params: HashMap<String, String>) -> RouterResult;
    /// Returns a tree of routes by filters
    fn router_map(&self) -> Option<RouterMap> {
        None
    }
}

impl RouterResult {
    pub fn is_handled(&self) -> bool {
        match self {
            RouterResult::Handled(_) => true,
            _ => false,
        }
    }

    pub fn is_unhandled(&self) -> bool {
        match self {
            RouterResult::Unhandled(_,_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum RouterErrorKind {
    #[fail(display="an error occured while handling request")]
    InnerError,
}

#[derive(Debug)]
pub struct RouterError {
    inner: Context<RouterErrorKind>,
}

impl_fail_boilerplate!(RouterErrorKind, RouterError);
