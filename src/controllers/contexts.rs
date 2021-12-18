use crate::{
    errors::InternalError,
    types::{Context, Flash},
};
use askama::Template;
use axum::response::Html;

pub async fn index() -> Result<Html<String>, InternalError> {
    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        contexts: vec![],
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/index.html")]
struct IndexTemplate {
    flash: Flash,
    contexts: Vec<Context>,
}
