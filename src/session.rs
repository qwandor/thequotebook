use crate::{config::Config, errors::InternalError, model::User};
use axum::{
    async_trait,
    body::Body,
    extract::{Extension, FromRequest, OriginalUri, RequestParts},
};
use eyre::eyre;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::{
    sync::Arc,
    time::{SystemTime, SystemTimeError},
};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Eq, PartialEq)]
pub struct Session {
    pub flash: Flash,
    pub current_user: Option<User>,
    // The path of the current page.
    pub path: String,
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

#[async_trait]
impl FromRequest<Body> for Session {
    type Rejection = InternalError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request(req)
            .await
            .map_err(|(_, e)| InternalError::Internal(eyre!("{}", e)))?;
        let Extension(config) = Extension::<Arc<Config>>::from_request(req).await?;
        let Extension(pool) = Extension::<Pool<Postgres>>::from_request(req).await?;
        let OriginalUri(uri) = OriginalUri::from_request(req).await?;
        let current_user = user_from_cookies(&config, &pool, cookies).await;
        Ok(Session {
            flash: Flash::from_request(req).await?,
            current_user,
            path: uri
                .path_and_query()
                .ok_or_else(|| InternalError::Internal(eyre!("Request URI missing path")))?
                .to_string(),
        })
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Flash {
    pub notice: Option<String>,
    pub error: Option<String>,
}

#[async_trait]
impl FromRequest<Body> for Flash {
    type Rejection = InternalError;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request(req)
            .await
            .map_err(|(_, e)| InternalError::Internal(eyre!("{}", e)))?;

        // If notice or error cookies exist, show them once then remove them.
        let notice = cookies.get("notice").map(|c| {
            cookies.remove(Cookie::new("notice", ""));
            c.value().to_owned()
        });
        let error = cookies.get("error").map(|c| {
            cookies.remove(Cookie::new("error", ""));
            c.value().to_owned()
        });

        Ok(Flash { notice, error })
    }
}

async fn user_from_cookies(
    config: &Config,
    pool: &Pool<Postgres>,
    cookies: Cookies,
) -> Option<User> {
    let session_token = cookies.get("session")?;
    let key = DecodingKey::from_secret(config.secret.as_bytes());
    let validation = Validation::default();
    let data = decode::<SessionClaims>(session_token.value(), &key, &validation).ok()?;
    User::fetch_one(pool, data.claims.sub).await.ok()
}

/// Claims for our session token.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionClaims {
    iat: u64,
    exp: u64,
    /// user_id
    sub: i32,
}

impl SessionClaims {
    pub fn new(
        user_id: i32,
        issued: SystemTime,
        expiry: SystemTime,
    ) -> Result<Self, SystemTimeError> {
        Ok(Self {
            sub: user_id,
            iat: issued.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            exp: expiry.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
        })
    }
}
