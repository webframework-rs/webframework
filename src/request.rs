use hyper::{self, Body, Uri};

pub struct Request {
    req: hyper::Request<Body>
}

impl Request {
    pub fn from_req(req: hyper::Request<Body>) -> Request {
        Request {
            req
        }
    }

    pub fn inner_req(&self) -> &hyper::Request<Body> {
        &self.req
    }

    pub fn path(&self) -> &Uri {
        self.req.uri()
    }
}

