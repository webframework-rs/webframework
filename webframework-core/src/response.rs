use crate::WebResult;

use failure::{Context, Fail, Compat, Backtrace, Error};
pub use http::StatusCode;
use hyper;
use futures::future::{self, Future};
use futures::Stream;

#[derive(Debug, Fail)]
enum ResponseErrorKind {
    #[fail(display = "unknown error")]
    UnknownError,
}

#[derive(Debug)]
struct ResponseError {
    inner: Context<ResponseErrorKind>,
}

impl ::std::fmt::Display for ResponseError {
    fn fmt(&self, f:  &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self.inner, f)
    }
}

impl Fail for ResponseError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

enum ResponseBody {
    Text(String),
}

impl ResponseBody {
    fn as_bytes_fut(self) -> impl Future<Item = Vec<u8>, Error = Error> {
        match self {
            ResponseBody::Text(s) => future::ok(s.into_bytes()),
        }
    }
}

pub struct Response {
    status: Option<StatusCode>,
    body: Option<ResponseBody>,
}

impl Response {
    pub fn from_string<S: ToString>(cont: S) -> Response {
        Response {
            status: None,
            body: Some(ResponseBody::Text(cont.to_string())),
        }
    }

    pub fn with_status(self, status: StatusCode) -> Self {
        Response {
            status: Some(status),
            ..self
        }
    }

    pub fn as_response(self) -> WebResult<hyper::Response<hyper::Body>> {
        let mut resp = hyper::Response::builder();

        if let Some(status) = self.status {
            resp.status(status);
        }

        if let Some(body) = self.body {
            Ok(resp.body(hyper::Body::wrap_stream(body.as_bytes_fut().into_stream().map_err(|e| {
                e.compat()
            })))?)
        } else {
            Ok(resp.body(hyper::Body::empty())?)
        }
    }
}

impl<'a> From<&'a str> for Response {
    fn from(s: &'a str) -> Response {
        Response::from_string(s)
    }
}

impl From<String> for Response {
    fn from(s: String) -> Response {
        Response::from_string(s)
    }
}
