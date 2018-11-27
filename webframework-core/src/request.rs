use hyper::{self, Body, Uri};
use http::method::Method;
use slog::Logger;
use uuid::Uuid;

pub struct Request {
    id: Uuid,
    log: Logger,
    req: hyper::Request<Body>,
}

impl Request {
    pub fn from_req(id: Uuid, log: Logger, req: hyper::Request<Body>) -> Request {
        Request {
            id, log, req
        }
    }

    pub fn inner_req(&self) -> &hyper::Request<Body> {
        &self.req
    }

    pub fn uri(&self) -> &Uri {
        self.req.uri()
    }

    pub fn path(&self) -> &str {
        self.req.uri().path()
    }

    pub fn method(&self) -> &Method {
        self.req.method()
    }

    pub fn log(&self) -> &Logger {
        &self.log
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

pub trait FromParameter: Sized {
    fn from_parameter(param: &str) -> crate::WebResult<Self>;
}

impl FromParameter for String {
    fn from_parameter(param: &str) -> crate::WebResult<Self> {
        Ok(param.to_string())
    }
}

pub trait FromRequest<'a>: Sized {
    fn from_request(req: &'a Request) -> crate::WebResult<Self>;
}

impl<'a> FromRequest<'a> for &'a Request {
    fn from_request(req: &Request) -> crate::WebResult<&Request> {
        Ok(req)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum RequestError {
    #[fail(display = "Could not find required param: _1")]
    ParamNotFound(String),
}
