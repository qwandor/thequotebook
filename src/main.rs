mod controllers;
mod errors;
mod filters;
mod session;
mod types;

use axum::{
    routing::{get, get_service},
    AddExtensionLayer, Router,
};
use controllers::{comments, contexts, home, quotes, users};
use errors::internal_error;
use eyre::Report;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Report> {
    stable_eyre::install()?;
    pretty_env_logger::init();
    color_backtrace::install();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://qwandor:password@localhost/quoteyou")
        .await?;

    let app = Router::new()
        .route("/", get(home::index))
        .route("/comments", get(home::comments))
        .route("/contexts", get(contexts::index))
        .route("/contexts/:context_id", get(contexts::show))
        .route("/users", get(users::index))
        .route("/users/:user_id", get(users::show))
        .route("/quotes", get(quotes::index))
        .route("/quotes/:quote_id", get(quotes::show))
        .route("/quotes/:quote_id/comments", get(comments::index))
        .route(
            "/quotes/:quote_id/comments/:comment_id",
            get(comments::show),
        )
        .nest(
            "/images",
            get_service(ServeDir::new("public/images")).handle_error(internal_error),
        )
        .nest(
            "/stylesheets",
            get_service(ServeDir::new("public/stylesheets")).handle_error(internal_error),
        )
        .layer(AddExtensionLayer::new(pool));

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
