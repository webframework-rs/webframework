use crate::request::{FromRequest, Request};
use crate::WebResult;

use serde::de::DeserializeOwned;
use failure::{Fail, Context, ResultExt};

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum FormErrorKind {
    #[fail(display = "An error occured during deserialization")]
    DeserializationError,
    #[fail(display = "The Content-Type could not be determined")]
    UnknownContentTypeError,
    #[fail(display = "The Content-Type is not supported: {}", format)]
    UnsupportedContentTypeError {
        format: String
    },
}

#[derive(Debug)]
pub struct FormError {
    inner: Context<FormErrorKind>,
}

impl_fail_boilerplate!(FormErrorKind, FormError);


#[derive(Debug, Clone)]
pub struct Form<'a, T> {
    req: &'a Request,
    typ: std::marker::PhantomData<T>,
}

impl<'a, T: DeserializeOwned> Form<'a, T> {
    pub fn get<S: AsRef<str>>(&self, _path: S) -> WebResult<T> {

        let content_type = self.req.headers().get(http::header::CONTENT_TYPE)
            .ok_or(FormErrorKind::UnknownContentTypeError.into())
            .and_then(|val| val.to_str().context(FormErrorKind::UnknownContentTypeError))?;

        match content_type {
            "application/x-www-form-urlencoded" => {
                Ok(serde_urlencoded::from_bytes(
                        self.req.body()).context(FormErrorKind::DeserializationError
                  )?)
            }

            kind => {
                Err(FormErrorKind::UnsupportedContentTypeError{ format: kind.into() })?
            }
        }
    }
}

impl<'a, T: DeserializeOwned> FromRequest<'a> for Form<'a, T> {
    fn from_request(req: &'a Request) -> WebResult<Form<'a, T>> {
        Ok(Form { req, typ: std::marker::PhantomData })
    }
}
