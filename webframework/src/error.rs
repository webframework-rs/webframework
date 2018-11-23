use failure::{Context, Fail};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct ServiceError {
    inner: Context<ServiceErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ServiceErrorKind {
    #[fail(display = "An Error spawning a new service")]
    ServiceCreation,
    #[fail(display = "An Error during handling a request")]
    RequestError,
}

impl ServiceError {
    pub fn kind(&self) -> ServiceErrorKind {
        *self.inner.get_context()
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

impl std::error::Error for ServiceError {}

