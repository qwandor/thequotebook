mod atom;
mod config;
mod controllers;
mod errors;
mod filters;
mod markdown;
mod model;
mod pagination;
mod responses;
mod session;

use axum::{
    extract::Extension,
    routing::{get, get_service, post},
    Router,
};
use config::Config;
use controllers::{comments, contexts, home, quotes, sessions, users};
use errors::internal_error;
use eyre::Report;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Report> {
    stable_eyre::install()?;
    pretty_env_logger::init();
    color_backtrace::install();

    let config = Arc::new(Config::from_file()?);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres_uri)
        .await?;

    let app = Router::new()
        .route("/", get(home::index))
        .route("/login", get(sessions::new))
        .route("/logout", get(sessions::destroy))
        .route("/google_auth", post(sessions::google_auth))
        .route("/comments", get(home::comments))
        .route("/comments.atom", get(home::comments_atom))
        .route("/contexts", get(contexts::index))
        .route("/contexts/new", get(contexts::new))
        .route("/contexts/:context_id", get(contexts::show))
        .route("/contexts/:context_id/edit", get(contexts::edit))
        .route("/contexts/:context_id/latest", get(contexts::latest))
        .route("/contexts/:context_id/join", post(contexts::join))
        .route("/contexts/:context_id/leave", post(contexts::leave))
        .route("/contexts/:context_id/quotes", get(contexts::quotes))
        .route(
            "/contexts/:context_id/quotes.atom",
            get(contexts::quotes_atom),
        )
        .route("/users", get(users::index))
        .route("/users/:user_id", get(users::show))
        .route("/users/:user_id/quotes", get(users::quotes))
        .route("/users/:user_id/quotes.atom", get(users::quotes_atom))
        .route(
            "/users/:user_id/relevant_quotes",
            get(users::relevant_quotes),
        )
        .route(
            "/users/:user_id/relevant_quotes.atom",
            get(users::relevant_quotes_atom),
        )
        .route(
            "/users/:user_id/relevant_comments",
            get(users::relevant_comments),
        )
        .route(
            "/users/:user_id/relevant_comments.atom",
            get(users::relevant_comments_atom),
        )
        .route("/users/:user_id/edit", get(users::edit))
        .route("/quotes", get(quotes::index))
        .route("/quotes.atom", get(quotes::index_atom))
        .route("/quotes/new", get(quotes::new))
        .route("/quotes/:quote_id", get(quotes::show))
        .route("/quotes/:quote_id/edit", get(quotes::edit))
        .route("/quotes/:quote_id/comments", get(comments::index))
        .route("/quotes/:quote_id/comments.atom", get(comments::index_atom))
        .route(
            "/quotes/:quote_id/comments/:comment_id",
            get(comments::show),
        )
        .nest_service(
            "/images",
            get_service(ServeDir::new(config.public_dir.join("images")))
                .handle_error(internal_error),
        )
        .nest_service(
            "/stylesheets",
            get_service(ServeDir::new(config.public_dir.join("stylesheets")))
                .handle_error(internal_error),
        )
        .layer(CookieManagerLayer::new())
        .layer(Extension(config.clone()))
        .layer(Extension(pool));

    info!("Listening on {}", config.bind_address);
    let listener = TcpListener::bind(&config.bind_address).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
