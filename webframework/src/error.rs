use failure::{Context, Fail, Backtrace};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Fail)]
pub enum ServiceErrorKind {
    #[fail(display = "An Error spawning a new service")]
    ServiceCreation,
    #[fail(display = "An Error during handling a request")]
    RequestError,
    #[fail(display = "Unhandled request for path: {}", _0)]
    UnhandledError(String),
    #[fail(display = "Internal Error")]
    InternalError,
}

#[derive(Debug)]
pub struct ServiceError {
    inner: Context<ServiceErrorKind>,
}

impl Fail for ServiceError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl ServiceError {
    pub fn kind(&self) -> ServiceErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<ServiceErrorKind> for ServiceError {
    fn from(kind: ServiceErrorKind) -> ServiceError {
        ServiceError { inner: Context::new(kind) }
    }
}

impl From<Context<ServiceErrorKind>> for ServiceError {
    fn from(inner: Context<ServiceErrorKind>) -> ServiceError {
        ServiceError { inner: inner }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
