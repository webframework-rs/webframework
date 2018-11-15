#![allow(non_snake_case)]
use crate::request::Request;

pub trait PathFilter: RequestFilter {
    fn name(&self) -> String;
}

impl<'a> PathFilter for &'a str {
    fn name(&self) -> String {
        self.to_string()
    }
}

pub trait RequestFilter {
    fn handles(&self, req: &Request) -> bool;
}

impl<'a> RequestFilter for &'a str {
    fn handles(&self, _req: &Request) -> bool {
        true
    }
}

impl<F> RequestFilter for F where F: Fn(&Request) -> bool {
    fn handles(&self, req: &Request) -> bool {
        self(req)
    }
}

pub struct NotFound;

impl RequestFilter for NotFound {
    fn handles(&self, _req: &Request) -> bool {
        true
    }
}

impl PathFilter for NotFound {
    fn name(&self) -> String {
        "NotFound".to_string()
    }
}

pub fn POST(_req: &Request) -> bool {
    true
}

pub fn GET(_req: &Request) -> bool {
    true
}

pub fn delegate(_: &Request) -> bool {
    true
}

pub fn html(_req: &Request) -> bool {
    true
}

pub fn json(_req: &Request) -> bool {
    true
}

