use std::sync::Arc;

use crate::{
    atom::comments::comments_to_atom,
    config::Config,
    errors::InternalError,
    filters,
    model::{CommentWithQuote, CommentWithQuotee, Quote},
    responses::Atom,
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::{Pool, Postgres};

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(quote_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let quote = Quote::fetch_one(&pool, quote_id).await?;
    let comments = CommentWithQuote::fetch_all_for_quote(&pool, quote_id).await?;

    let template = IndexTemplate {
        session,
        quote,
        comments,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "comments/index.html")]
struct IndexTemplate {
    session: Session,
    quote: Quote,
    comments: Vec<CommentWithQuote>,
}

pub async fn index_atom(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    Path(quote_id): Path<i32>,
) -> Result<Atom, InternalError> {
    let quote = Quote::fetch_one(&pool, quote_id).await?;
    let comments = CommentWithQuotee::fetch_all_for_quote(&pool, quote_id).await?;
    let title = format!("theQuotebook: Comments on {}", quote.quote_text);
    let path = format!("/quotes/{}/comments", quote_id);

    Ok(Atom(comments_to_atom(comments, title, &path, &config)?))
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path((quote_id, comment_id)): Path<(i32, i32)>,
) -> Result<Html<String>, InternalError> {
    let comment = CommentWithQuote::fetch_one(&pool, quote_id, comment_id).await?;

    let template = ShowTemplate { session, comment };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "comments/show.html")]
struct ShowTemplate {
    session: Session,
    comment: CommentWithQuote,
}
