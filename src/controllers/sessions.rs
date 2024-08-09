use crate::{
    config::Config,
    errors::InternalError,
    model::User,
    session::{Session, SessionClaims},
};
use askama::{filters::urlencode, Template};
use axum::{
    extract::{Extension, Form, Query},
    http::Uri,
    response::{Html, IntoResponse, Redirect, Response},
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
    Query(query): Query<RedirectQuery>,
) -> Result<Html<String>, InternalError> {
    let auth_url = if let Some(redirect) = query.redirect {
        format!(
            "{}/google_auth?redirect={}",
            config.base_url,
            urlencode(redirect)?
        )
    } else {
        format!("{}/google_auth", config.base_url)
    };
    let template = NewTemplate {
        session,
        google_client_id: config.google_client_id.to_owned(),
        auth_url,
    };
    Ok(Html(template.render()?))
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedirectQuery {
    redirect: Option<String>,
}

#[derive(Template)]
#[template(path = "sessions/new.html")]
struct NewTemplate {
    session: Session,
    google_client_id: String,
    auth_url: String,
}

pub async fn google_auth(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    Query(query): Query<RedirectQuery>,
    cookies: Cookies,
    Form(request): Form<GoogleAuthRequest>,
) -> Result<Response, InternalError> {
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
        let key = EncodingKey::from_secret(config.secret.as_bytes());
        let header = Header::default();
        let now = SystemTime::now();
        let expiry = now + config.session_duration;
        let claims = SessionClaims::new(user.id, now, expiry)?;
        let token = encode(&header, &claims, &key)?;

        // TODO: Set Secure, once we enforce https.
        cookies.add(
            Cookie::build("session", token)
                .http_only(true)
                .max_age(config.session_duration.try_into()?)
                .finish(),
        );

        cookies.add(Cookie::new("notice", "Logged in successfully."));

        let redirect: Uri = query.redirect.as_deref().unwrap_or("/").parse()?;
        if redirect.host().is_some() || redirect.scheme().is_some() {
            return Err(InternalError::Internal(eyre!("Invalid redirect path")));
        }
        Ok(Redirect::to(&redirect.to_string()).into_response())
    } else {
        // TODO: Redirect to the account creation form.
        Ok(format!("No such user: {:?}", google_claims).into_response())
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

pub async fn destroy(
    cookies: Cookies,
    Query(query): Query<RedirectQuery>,
) -> Result<Redirect, InternalError> {
    cookies.remove(Cookie::new("session", ""));

    cookies.add(Cookie::new("notice", "You have been logged out."));

    let redirect: Uri = query.redirect.as_deref().unwrap_or("/").parse()?;
    if redirect.host().is_some() || redirect.scheme().is_some() {
        return Err(InternalError::Internal(eyre!("Invalid redirect path")));
    }
    Ok(Redirect::to(&redirect.to_string()))
}
