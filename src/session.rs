use crate::{config::Config, errors::InternalError, model::User};
use axum::{
    async_trait,
    body::Body,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
};
use headers::Cookie;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

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
    type Rejection = InternalError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        // TODO: Don't return an error if there are no cookies.
        let TypedHeader(cookies) = TypedHeader::<Cookie>::from_request(req).await?;
        let Extension(config) = Extension::<Arc<Config>>::from_request(req).await?;
        let Extension(pool) = Extension::<Pool<Postgres>>::from_request(req).await?;
        let current_user = if let Some(session_token) = cookies.get("session") {
            let key = DecodingKey::from_secret(&config.secret.as_ref());
            let validation = Validation {
                validate_exp: false,
                ..Validation::default()
            };
            // TODO: On error, just unset cookie and return None.
            let data = decode::<SessionClaims>(session_token, &key, &validation)?;
            User::fetch_one(&pool, data.claims.sub).await.ok()
        } else {
            None
        };
        Ok(Session {
            flash: Flash::default(),
            current_user,
        })
    }
}

/// Claims for our session token.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionClaims {
    pub iat: u64,
    /// user_id
    pub sub: i32,
    // TODO: Add exp?
}
