mod contexts;
mod filters;
mod home;
mod types;

use axum::{
    http::StatusCode,
    routing::{get, get_service},
    AddExtensionLayer, Router,
};
use eyre::Report;
use sqlx::postgres::PgPoolOptions;
use std::io;
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
        .nest(
            "/images",
            get_service(ServeDir::new("public/images")).handle_error(
                |error: io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .nest(
            "/stylesheets",
            get_service(ServeDir::new("public/stylesheets")).handle_error(
                |error: io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .layer(AddExtensionLayer::new(pool));

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
