use crate::{
    config::Config,
    errors::InternalError,
    model::User,
    session::{Session, SessionClaims},
};
use askama::Template;
use axum::{
    extract::{Extension, Form},
    response::Html,
};
use eyre::eyre;
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken_google::Parser;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::{sync::Arc, time::SystemTime};
use tower_cookies::{Cookie, Cookies};

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
    Form(request): Form<GoogleAuthRequest>,
    cookies: Cookies,
) -> Result<String, InternalError> {
    if request.g_csrf_token
        != cookies
            .get("g_csrf_token")
            .ok_or(InternalError::Internal(eyre!("Missing CSRF token")))?
            .value()
    {
        return Err(InternalError::Internal(eyre!("Invalid CSRF token")));
    }

    // Validate JWT and parse claims.
    // See https://developers.google.com/identity/gsi/web/guides/verify-google-id-token
    let parser = Parser::new(&config.google_client_id);
    let google_claims = parser.parse::<TokenClaims>(&request.credential).await?;

    if !google_claims.email_verified {
        return Err(InternalError::Internal(eyre!("Email not verified")));
    }

    // User has successfully authenticated with Google, see if they exist in our database.
    if let Some(user) = User::fetch_by_email(&pool, &google_claims.email).await? {
        // Issue a JWT for the user.
        let key = EncodingKey::from_secret(&config.secret.as_ref());
        let header = Header::default();
        let claims = SessionClaims {
            sub: user.id,
            iat: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
        };
        let token = encode(&header, &claims, &key)?;
        let response = format!("Successfully logged in: {:?}, {}", google_claims, token);

        // TODO: Set Expires or Max-Age so that cookie lasts longer than session.
        // TODO: Set Secure, once we enforce https.
        cookies.add(Cookie::build("session", token).http_only(true).finish());

        Ok(response)
    } else {
        // TODO: Redirect to the account creation form.
        Ok(format!("No such user: {:?}", google_claims))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GoogleAuthRequest {
    credential: String,
    g_csrf_token: String,
}

/// Claims from Google login.
#[derive(Debug, Deserialize)]
struct TokenClaims {
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}
