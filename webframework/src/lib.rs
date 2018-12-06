extern crate webframework_derive;
extern crate webframework_core;
extern crate failure;
extern crate hyper;
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate regex;
#[macro_use] extern crate horrorshow;

pub mod prelude;
pub mod request_filter;
pub mod server;
pub mod error;
mod templates;

pub use webframework_derive::controller;
pub use webframework_derive::routing;
