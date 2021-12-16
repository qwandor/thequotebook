mod index;

use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use eyre::Report;
use std::io;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Report> {
    stable_eyre::install()?;
    pretty_env_logger::init();
    color_backtrace::install();

    let app = Router::new()
        .route("/", get(index::handle))
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
        );

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
