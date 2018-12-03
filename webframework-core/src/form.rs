use crate::request::{FromRequest, Request};
use crate::WebResult;

use futures::{Stream, Future};
use serde::de::DeserializeOwned;
use failure::Fail;

pub struct Form<'a, T> {
    req: &'a Request,
    typ: std::marker::PhantomData<T>,
}

impl<'a, T: DeserializeOwned> Form<'a, T> {
    pub fn get<S: AsRef<str>>(&self, path: S) -> WebResult<T> {
        unimplemented!()
    }
}

impl<'a, T: DeserializeOwned> FromRequest<'a> for Form<'a, T> {
    fn from_request(req: &'a Request) -> WebResult<Form<'a, T>> {
        Ok(Form { req, typ: std::marker::PhantomData })
    }
}
