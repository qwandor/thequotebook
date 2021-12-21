mod config;
mod controllers;
mod errors;
mod filters;
mod model;
mod pagination;
mod session;

use axum::{
    routing::{get, get_service},
    AddExtensionLayer, Router,
};
use config::Config;
use controllers::{comments, contexts, home, quotes, users};
use errors::internal_error;
use eyre::Report;
use log::info;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Report> {
    stable_eyre::install()?;
    pretty_env_logger::init();
    color_backtrace::install();

    let config = Config::from_file()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres_uri)
        .await?;

    let app = Router::new()
        .route("/", get(home::index))
        .route("/comments", get(home::comments))
        .route("/contexts", get(contexts::index))
        .route("/contexts/:context_id", get(contexts::show))
        .route("/contexts/:context_id/quotes", get(contexts::quotes))
        .route("/users", get(users::index))
        .route("/users/:user_id", get(users::show))
        .route("/users/:user_id/quotes", get(users::quotes))
        .route("/quotes", get(quotes::index))
        .route("/quotes/:quote_id", get(quotes::show))
        .route("/quotes/:quote_id/comments", get(comments::index))
        .route(
            "/quotes/:quote_id/comments/:comment_id",
            get(comments::show),
        )
        .nest(
            "/images",
            get_service(ServeDir::new(config.public_dir.join("images")))
                .handle_error(internal_error),
        )
        .nest(
            "/stylesheets",
            get_service(ServeDir::new(config.public_dir.join("stylesheets")))
                .handle_error(internal_error),
        )
        .layer(AddExtensionLayer::new(pool));

    info!("Listening on {}", config.bind_address);
    axum::Server::bind(&config.bind_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
