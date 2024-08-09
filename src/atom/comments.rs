use crate::{
    config::Config,
    errors::InternalError,
    filters,
    model::{Comment, CommentWithQuotee, User},
};
use askama::Template;
use atom_syndication::{
    ContentBuilder, Entry, EntryBuilder, Feed, FeedBuilder, GeneratorBuilder, LinkBuilder,
    PersonBuilder,
};
use chrono::{DateTime, Utc};

pub fn comments_to_atom(
    comments: Vec<CommentWithQuotee>,
    title: String,
    path: &str,
    config: &Config,
) -> Result<Feed, InternalError> {
    let last_updated = comments
        .iter()
        .map(|comment| comment.comment.updated_at)
        .max()
        .unwrap_or(DateTime::<Utc>::MIN_UTC);
    let feed_url = format!("{}{}.atom", config.base_url, path);
    Ok(FeedBuilder::default()
        .title(title)
        .link(
            LinkBuilder::default()
                .rel("self")
                .mime_type("application/atom+xml".to_string())
                .href(&feed_url)
                .build(),
        )
        .link(
            LinkBuilder::default()
                .rel("alternate")
                .mime_type("text/html".to_string())
                .href(config.absolute_url(path))
                .build(),
        )
        .id(feed_url)
        .generator(
            GeneratorBuilder::default()
                .value("theQuotebook")
                .uri(config.absolute_url("/"))
                .build(),
        )
        .updated(last_updated)
        .entries(
            comments
                .into_iter()
                .map(|comment| comment_to_atom(&config.base_url, comment))
                .collect::<Result<Vec<_>, InternalError>>()?,
        )
        .build())
}

fn comment_to_atom(base_url: &str, comment: CommentWithQuotee) -> Result<Entry, InternalError> {
    let url = format!(
        "{}/quotes/{}/comments/{}",
        base_url, comment.comment.quote_id, comment.comment.id
    );
    Ok(EntryBuilder::default()
        .title(format!(
            "{} on {} ({})",
            comment.user.username_or_fullname(),
            comment.quote_text,
            comment.quotee.fullname,
        ))
        .link(
            LinkBuilder::default()
                .rel("alternate")
                .mime_type("text/html".to_string())
                .href(&url)
                .build(),
        )
        .id(url)
        .updated(comment.comment.updated_at)
        .published(Some(comment.comment.created_at.into()))
        .author(
            PersonBuilder::default()
                .name(comment.user.username_or_fullname())
                .uri(format!("{}/users/{}", base_url, comment.user.id))
                .build(),
        )
        .content(
            ContentBuilder::default()
                .content_type("html".to_string())
                .value(chatty_comment(comment, base_url)?)
                .build(),
        )
        .build())
}

fn chatty_comment(comment: CommentWithQuotee, base_url: &str) -> askama::Result<String> {
    let template = ChattyCommentTemplate {
        comment: comment.comment,
        quote_text: comment.quote_text,
        user: comment.user,
        quotee: comment.quotee,
        base_url: base_url.to_owned(),
    };

    template.render()
}

#[derive(Template)]
#[template(path = "shared/chatty_comment.html")]
struct ChattyCommentTemplate {
    comment: Comment,
    quote_text: String,
    user: User,
    quotee: User,
    base_url: String,
}
