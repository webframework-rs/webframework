extern crate futures;
#[macro_use] extern crate failure;

pub mod request;
pub mod request_filter;
pub mod response;
pub mod router;

pub type WebResult<T> = ::std::result::Result<T, failure::Error>;
