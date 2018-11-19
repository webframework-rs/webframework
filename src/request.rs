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

