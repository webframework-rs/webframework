use crate::WebResult;
use crate::router::Router;
use crate::request::Request;
use crate::error::{ServiceError, ServiceErrorKind};

use failure;
use futures::future::{self, Future, FutureResult};
use std::net::SocketAddr;

struct ServiceCreator<S: Router> {
    router: S,
}

impl<S: Router + 'static + Send> hyper::service::NewService for ServiceCreator<S> {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = ServiceError;
    type InitError = ServiceError;
    type Service = Service<S>;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        future::ok(Service {
            router: self.router.clone()
        })
    }
}

struct Service<S: Router> {
    router: S,
}

impl<S: Router + 'static + Send> hyper::service::Service for Service<S> {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = ServiceError;
    type Future = Box<dyn Future<Item = hyper::Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        let request = Request::from_req(req);
        Box::new(self.router.handle(request).and_then(|resp| {
            resp.as_response()
        }).or_else(|e: failure::Error| {
            future::err(e.context(ServiceErrorKind::RequestError).into())
        }))
    }
}


pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn handle_with<S: Router + Send + Sync + 'static>(&self, router: S) -> WebResult<()> {
        let creator = ServiceCreator {
            router
        };

        let server = hyper::Server::bind(&self.addr)
            .serve(creator)
            .map_err(|err| eprintln!("error: {}", err));

        hyper::rt::run(server);
        Ok(())
    }
}

pub fn load() -> Server {
    Server {
        addr: ([127, 0, 0, 1], 3000).into(),
    }
}
