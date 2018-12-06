use hyper::{self, Body, Uri};
use http::method::Method;
use http::request::Parts;
use http::header::{HeaderMap, HeaderValue};
use slog::Logger;
use uuid::Uuid;
use futures::{Future, Stream};
use bytes::Bytes;
use failure::{Fail, Context, Error};

#[derive(Debug)]
pub struct Request {
    id: Uuid,
    log: Logger,
    body: Bytes,
    parts: Parts,
}

impl Request {
    pub fn from_req(id: Uuid, log: Logger, req: hyper::Request<Body>)
        -> impl Future<Item = Request, Error = Error> + Send + Sized
    {
        let (parts, body) = req.into_parts();

        body.concat2().and_then(move |body| {
            let body = body.into_bytes();

            futures::future::ok(Request {
                id, log, body, parts
            })
        }).map_err(|e| e.context(RequestErrorKind::BodyParseError).into())
    }

    pub fn uri(&self) -> &Uri {
        &self.parts.uri
    }

    pub fn path(&self) -> &str {
        self.parts.uri.path()
    }

    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.parts.headers
    }

    pub fn log(&self) -> &Logger {
        &self.log
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn body(&self) -> &[u8] {
        &self.body
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
pub enum RequestErrorKind {
    #[fail(display = "Could not find required param: _1")]
    ParamNotFound(String),
    #[fail(display="could not parse the body")]
    BodyParseError
}

#[derive(Debug)]
pub struct RequestError {
    inner: Context<RequestErrorKind>,
}

impl_fail_boilerplate!(RequestErrorKind, RequestError);
