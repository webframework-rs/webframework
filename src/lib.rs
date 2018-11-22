extern crate webframework_derive;
#[macro_use] extern crate failure;
extern crate hyper;
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate regex;

pub mod prelude;
pub mod request;
pub mod response;
pub mod request_filters;
pub mod server;
pub mod router;
pub mod error;

pub use webframework_derive::controller;
pub use webframework_derive::meta_controller;
pub use webframework_derive::routing;

pub type WebResult<T> = ::std::result::Result<T, failure::Error>;
