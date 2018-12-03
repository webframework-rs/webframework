#![allow(non_snake_case, non_camel_case_types)]
use webframework_core::request::Request;
use webframework_core::request_filter::{PathFilter, PathFilterResult};

use std::collections::HashMap;

use http;
use http::method::Method;
use regex::Regex;

pub struct PathRegex(Regex);

impl PathRegex {
    pub fn from_regex(reg: Regex) -> PathRegex {
        PathRegex(reg)
    }
}

impl PathFilter for PathRegex {
    fn handles(&self, _req: &Request, path: &str) -> PathFilterResult {
        let result = self.0.is_match(path);

        if result {
            let captures = self.0.captures(path).unwrap();
            let end = captures.get(1).unwrap().end();

            let mut params = HashMap::new();

            params.extend({
                self.0.capture_names().flat_map(|name| {
                    name.map(|name| {
                        (String::from(name), captures.name(name).unwrap().as_str().to_string())
                    })
                })
            });


            let mut new_path = (&path[end..]).to_string();

            if new_path.is_empty() || new_path == "/" {
                new_path = String::from("/");
            }

            PathFilterResult::Matched(new_path, params)
        } else {
            PathFilterResult::NotMatched
        }
    }

    fn name(&self) -> String {
        self.0.to_string()
    }
}

macro_rules! request_filter {
    ( $name:ident, $desc:expr => $req:ident $impl:block ) => {
        pub struct $name;

        impl webframework_core::request_filter::RequestFilter for $name {
            fn handles(&self, $req: &$crate::request_filter::Request) -> bool {
                let val: bool = $impl;

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
    delegate, "Delegates to the next filter or router" => _req {
        true
    }
}

request_filter! {
    html, "Requires the client to request HTML" => req {
        let _html_type = "text/html";
        let client_accept = req.headers().get(http::header::ACCEPT);
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
