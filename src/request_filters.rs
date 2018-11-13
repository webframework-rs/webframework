#![allow(non_snake_case)]
use crate::request::Request;

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

pub fn POST(_req: &Request) -> bool {
    true
}

pub fn GET(_req: &Request) -> bool {
    true
}

pub fn DLG(_: &Request) -> bool {
    true
}

pub fn NotFound(_: &Request) -> bool {
    true
}

pub fn html(_req: &Request) -> bool {
    true
}

pub fn json(_req: &Request) -> bool {
    true
}

