use crate::error::{ServiceError, ServiceErrorKind};

use webframework_core::WebResult;
use webframework_core::request::Request;
use webframework_core::router::{Router, RouterResult};

use std::net::SocketAddr;
use std::time::Instant;
use std::collections::HashMap;

use failure::{self, Compat, Fail};
use futures::future::{self, Future, FutureResult};
use uuid::Uuid;
use slog::{Drain, Logger};

struct ServiceCreator<S: Router> {
    logger: Logger,
    router: S,
}

impl<S: Router + 'static + Send> hyper::service::NewService for ServiceCreator<S> {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = Compat<ServiceError>;
    type InitError = Compat<ServiceError>;
    type Service = Service<S>;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        future::ok(Service {
            logger: self.logger.new(slog::o!()),
            router: self.router.clone(),
        })
    }
}

struct Service<S: Router> {
    logger: Logger,
    router: S,
}

impl<S: Router + 'static + Send> hyper::service::Service for Service<S> {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = Compat<ServiceError>;
    type Future = Box<dyn Future<Item = hyper::Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        let id = Uuid::new_v4();
        let string_id = id.to_string();
        let new_logger = self.logger.new(slog::o!("path" => req.uri().path().to_string(),
                                                   "id" => string_id.clone()));
        let time_logger = self.logger.new(slog::o!("path" => req.uri().path().to_string(),
                                                   "id" => string_id.clone()));
        let request = Request::from_req(id, new_logger, req);
        let now = Instant::now();
        let ret : Box<dyn Future<Item = _, Error = _> + Send> = match self.router.handle(request, None, HashMap::new()) {
            RouterResult::Handled(resp) => {
                Box::new(resp.and_then(|resp| {
                    resp.as_response()
                }).or_else(|e: failure::Error| {
                    let error: ServiceError = e.context(ServiceErrorKind::RequestError).into();
                    future::err(error.compat())
                }))
            }
            RouterResult::Unhandled(_, _) => {
                let error: ServiceError = ServiceErrorKind::RequestError.into();
                Box::new(future::err(error.compat()))
            }
        };

        Box::new(ret.then(move |resp| {
            let elapsed = now.elapsed();
            let time: f64 = elapsed.as_secs() as f64 * 1000.0 + elapsed.subsec_nanos() as f64 / 1_000_000.0;
            slog::info!(time_logger, "Handled in {}ms", time; "elapsed_time" => time);
            resp
        }))
    }
}


pub struct Server {
    root_logger: Logger,
    addr: SocketAddr,
}

impl Server {
    pub fn handle_with<S: Router + Send + Sync + 'static>(&self, router: S) -> WebResult<()> {
        let creator = ServiceCreator {
            logger: self.root_logger.new(slog::o!()),
            router
        };

        slog::info!(&creator.logger, "Starting Webserver on http://{}", &self.addr;
                    "port" => %self.addr.port(), "host" => %self.addr.ip());

        let server = hyper::Server::bind(&self.addr)
            .serve(creator)
            .map_err(|err| eprintln!("error: {}", err));

        hyper::rt::run(server);
        Ok(())
    }
}

pub fn load() -> Server {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_logger = Logger::root(drain, slog::o!());
    Server {
        root_logger,
        addr: ([127, 0, 0, 1], 3000).into(),
    }
}
