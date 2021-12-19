use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, RequestParts},
};
use std::convert::Infallible;

#[derive(Debug, Eq, PartialEq)]
pub struct Session {
    pub flash: Flash,
    pub logged_in: bool,
    pub current_user_id: i32,
    pub current_user_fullname: String,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Flash {
    pub notice: Option<String>,
    pub error: Option<String>,
}

#[async_trait]
impl FromRequest<Body> for Session {
    type Rejection = Infallible;

    async fn from_request(_req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        Ok(Session {
            flash: Flash::default(),
            logged_in: false,
            current_user_id: 1,
            current_user_fullname: "".to_string(),
        })
    }
}
