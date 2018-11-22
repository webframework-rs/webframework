#![allow(non_snake_case, non_camel_case_types)]
use crate::request::Request;

use std::collections::HashMap;

use http;
use http::method::Method;
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathFilterResult {
    NotMatched,
    Matched(String, HashMap<String, String>),
}

pub trait PathFilter {
    fn handles(&self, req: &Request, path: &str) -> PathFilterResult;
    fn name(&self) -> String;
}

impl PathFilter for Regex {
    fn handles(&self, _req: &Request, path: &str) -> PathFilterResult {
        let result = self.is_match(path);

        if result {
            let captures = self.captures(path).unwrap();
            let end = captures.get(1).unwrap().end();

            let mut params = HashMap::new();

            params.extend({
                self.capture_names().flat_map(|name| {
                    name.map(|name| {
                        (String::from(name), captures.name(name).unwrap().as_str().to_string())
                    })
                })
            });


            let mut new_path = (&path[end..]).to_string();

            if new_path.is_empty() || new_path == "/" {
                new_path = String::from("/");
            } else {
                return PathFilterResult::NotMatched;
            }

            PathFilterResult::Matched(new_path, params)
        } else {
            PathFilterResult::NotMatched
        }
    }

    fn name(&self) -> String {
        self.to_string()
    }
}

pub trait RequestFilter {
    fn handles(&self, req: &Request) -> bool;
    fn description() -> String;
}

macro_rules! request_filter {
    ( $name:ident, $desc:expr => $req:ident $impl:block ) => {
        pub struct $name;

        impl $crate::request_filters::RequestFilter for $name {
            fn handles(&self, $req: &$crate::request_filters::Request) -> bool {
                let val: bool = $impl;

                if val {
                    let log = $req.log();
                    slog::debug!(log, "filtered by {}", stringify!($name);"request_filter" => stringify!($name));
                }

                return val;
            }

            fn description() -> String {
                String::from($desc)
            }
        }
    };
}

request_filter! {
    POST, "Requires POST HTTP Method" => req {
        req.method() == Method::POST
    }
}

request_filter! {
    GET, "Requires POST GET Method" => req {
        req.method() == Method::GET
    }
}

request_filter! {
    delegate, "Delegates to the next filter or router" => req {
        true
    }
}

request_filter! {
    html, "Requires the client to request HTML" => req {
        let _html_type = "text/html";
        let client_accept = req.inner_req().headers().get(http::header::ACCEPT);
        if let Some(_header) = client_accept {
            false
        } else {
            false
        }
    }
}

request_filter! {
    json, "Requires the client to request JSON" => req {
        let _json_type = "application/javascript";
        req.method() == Method::GET
    }
}
