mod controllers;
mod errors;
mod filters;
mod types;

use axum::{
    routing::{get, get_service},
    AddExtensionLayer, Router,
};
use controllers::{contexts, home, users};
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
        .route("/users", get(users::index))
        .route("/users/:id", get(users::show))
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
