use crate::WebResult;

use failure::{Context, Fail, Error};
pub use http::StatusCode;
use http::header::{HeaderMap, HeaderValue, HeaderName};
use hyper;
use futures::future::{self, Future};
use futures::Stream;

#[derive(Debug, Fail, Clone)]
pub enum ResponseErrorKind {
    #[fail(display = "unknown error")]
    UnknownError,
    #[fail(display = "an error occurred with the inner Body type")]
    BodyError,
}

#[derive(Debug)]
pub struct ResponseError {
    inner: Context<ResponseErrorKind>,
}

impl_fail_boilerplate!(ResponseErrorKind, ResponseError);

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
    headers: HeaderMap,
}

impl Response {
    pub fn from_string<S: ToString>(cont: S) -> Response {
        Response {
            status: None,
            body: Some(ResponseBody::Text(cont.to_string())),
            headers: HeaderMap::new(),
        }
    }

    pub fn with_status(self, status: StatusCode) -> Self {
        Response {
            status: Some(status),
            ..self
        }
    }

    pub fn with_header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn as_response(self) -> WebResult<hyper::Response<hyper::Body>> {
        let mut resp = hyper::Response::builder();

        if let Some(status) = self.status {
            resp.status(status);
        }

        for (k, v) in &self.headers {
            resp.header(k, v);
        }

        if let Some(body) = self.body {
            Ok(resp.body(hyper::Body::wrap_stream(body.as_bytes_fut().into_stream().map_err(|e| {
                e.context(ResponseErrorKind::BodyError).compat()
            })))?)
        } else {
            Ok(resp.body(hyper::Body::empty())?)
        }
    }
}

pub struct Redirect(HeaderValue);

impl Redirect {
    pub fn from_str(val: &str) -> WebResult<Redirect> {
        Ok(Redirect(val.parse()?))
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

impl From<Redirect> for Response {
    fn from(r: Redirect) -> Response {
        Response::from_string("")
            .with_header("Location".parse().unwrap(), r.0)
            .with_status(StatusCode::SEE_OTHER)
    }
}
