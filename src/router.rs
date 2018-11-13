use crate::request::Request;
use crate::response::Response;

use failure::Error;
use futures::Future;

pub trait Router: Clone {
    fn handle(&self, req: Request) -> Box<dyn Future<Item = Response, Error = Error> + Send>;
}

impl<F: Clone> Router for F where F: Fn(Request) -> Box<dyn Future<Item = Response, Error = Error> + Send> {
    fn handle(&self, req: Request) -> Box<dyn Future<Item = Response, Error = Error> + Send> {
        self(req)
    }
}
