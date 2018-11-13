use crate::WebResult;
use crate::error::ServiceError;

pub use http::StatusCode;
use hyper;
use futures::future::{self, Future};

enum ResponseBody {
    Text(String),
}

impl ResponseBody {
    fn as_bytes_fut(self) -> impl Future<Item = Vec<u8>, Error = ServiceError> {
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
            Ok(resp.body(hyper::Body::wrap_stream(body.as_bytes_fut().into_stream()))?)
        } else {
            Ok(resp.body(hyper::Body::empty())?)
        }
    }
}

