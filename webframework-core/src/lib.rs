extern crate futures;
#[macro_use] extern crate failure;

#[macro_use] pub mod error;
pub mod request;
pub mod request_filter;
pub mod response;
pub mod router;
pub mod form;

pub type WebResult<T> = ::std::result::Result<T, failure::Error>;
