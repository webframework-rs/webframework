use crate::request::Request;

use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathFilterResult {
    NotMatched,
    Matched(String, HashMap<String, String>),
}

pub trait PathFilter {
    fn handles(&self, req: &Request, path: &str) -> PathFilterResult;
    fn name(&self) -> String;
}

pub trait RequestFilter {
    fn handles(&self, req: &Request) -> bool;
    fn description() -> String;
}


