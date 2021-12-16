use askama::Template;
use axum::response::Html;

pub async fn index() -> Result<Html<String>, String> {
    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        quotes: vec![],
        top_contexts: vec![],
        current_user_contexts: vec![],
        current_user_comments: vec![],
        current_user_id: 0,
        current_user_fullname: "".to_string(),
    };
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

pub async fn comments() -> Result<Html<String>, String> {
    let template = CommentsTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
    };
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct IndexTemplate {
    flash: Flash,
    logged_in: bool,
    quotes: Vec<String>,
    top_contexts: Vec<Context>,
    current_user_contexts: Vec<Context>,
    current_user_comments: Vec<String>,
    current_user_id: u32,
    current_user_fullname: String,
}

#[derive(Template)]
#[template(path = "home/comments.html")]
struct CommentsTemplate {
    flash: Flash,
}

struct Context {
    id: u32,
    name: String,
    description: String,
    quote_count: u32,
}

struct Flash {
    notice: Option<String>,
    error: Option<String>,
}
