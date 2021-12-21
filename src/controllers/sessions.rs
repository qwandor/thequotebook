use crate::{config::Config, errors::InternalError, session::Session};
use askama::Template;
use axum::{
    extract::{Extension, Form, TypedHeader},
    response::Html,
};
use eyre::eyre;
use headers::Cookie;
use jsonwebtoken_google::Parser;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub async fn new(
    Extension(config): Extension<Arc<Config>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let template = NewTemplate {
        session,
        google_client_id: config.google_client_id.to_owned(),
        base_url: config.base_url.to_owned(),
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "sessions/new.html")]
struct NewTemplate {
    session: Session,
    google_client_id: String,
    base_url: String,
}

pub async fn google_auth(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Form(request): Form<GoogleAuthRequest>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Result<String, InternalError> {
    if request.g_csrf_token != cookies.get("g_csrf_token").unwrap_or("") {
        return Err(InternalError::Internal(eyre!("Invalid CSRF token")));
    }

    // Validate JWT and parse claims.
    // See https://developers.google.com/identity/gsi/web/guides/verify-google-id-token
    let parser = Parser::new(&config.google_client_id);
    let claims = parser.parse::<TokenClaims>(&request.credential).await?;

    // User has successfully authenticated with Google, see if they exist in our database.

    Ok(format!("{:?}", claims))
}

#[derive(Clone, Debug, Deserialize)]
pub struct GoogleAuthRequest {
    credential: String,
    g_csrf_token: String,
}

#[derive(Debug, Deserialize)]
struct TokenClaims {
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}
