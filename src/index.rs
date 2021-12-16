use askama::Template;
use axum::response::Html;

pub async fn handle() -> Result<Html<String>, String> {
    let template = IndexTemplate {};
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
