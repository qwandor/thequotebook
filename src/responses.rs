use atom_syndication::Feed;
use axum::{
    body::{boxed, Full},
    http::{header, HeaderValue},
    response::{IntoResponse, Response},
};

#[derive(Clone, Debug)]
pub struct Atom(pub Feed);

impl IntoResponse for Atom {
    fn into_response(self) -> Response {
        let mut res = Response::new(boxed(Full::from(self.0.to_string())));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/atom+xml"),
        );
        res
    }
}
