use crate::model::User;
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, RequestParts},
};
use std::convert::Infallible;

#[derive(Debug, Eq, PartialEq)]
pub struct Session {
    pub flash: Flash,
    pub current_user: Option<User>,
}

impl Session {
    pub fn logged_in(&self) -> bool {
        self.current_user.is_some()
    }

    pub fn is_current_user(&self, &user_id: &i32) -> bool {
        if let Some(current_user) = &self.current_user {
            current_user.id == user_id
        } else {
            false
        }
    }
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
            current_user: None,
        })
    }
}
