use super::escape;
use crate::types::User;

pub fn link_to_user(
    user: &User,
    &avatar: &bool,
    &actually_link: &bool,
    &css_avatar: &bool,
    &avatar_size: &u16,
    prefix: &str,
) -> askama::Result<String> {
    let content_text = &user.username;
    let title = &user.fullname;
    let class = "nickname";
    let gravatar = if avatar && !css_avatar {
        gravatar(&user.email, avatar_size)
    } else {
        "".to_string()
    };
    let link_text = format!("{}{}{}", prefix, gravatar, escape(content_text));
    Ok(if actually_link {
        // TODO: Support full_url
        let user_url = format!("/user/{}", user.id);
        format!(
            "<a href=\"{}\" class=\"{}\" title=\"{}\">{}</a>",
            user_url, class, title, link_text
        )
    } else if css_avatar {
        let image = gravatar_url(&user.email, avatar_size);
        format!(
            "<span class=\"author\" style=\"background-image:url({})\">{}</span>",
            escape(&image),
            link_text
        )
    } else {
        link_text
    })
}

fn gravatar(email_address: &str, avatar_size: u16) -> String {
    let url = gravatar_url(email_address, avatar_size);
    format!(
        "<img class=\"gravatar\" alt=\"avatar\" width=\"{}\" height=\"{}\" src=\"{}\"/>",
        avatar_size,
        avatar_size,
        escape(&url)
    )
}

fn gravatar_url(email_address: &str, avatar_size: u16) -> String {
    let email_hash = md5::compute(email_address);
    format!(
        "http://www.gravatar.com/avatar.php?gravatar_id={:x}&avatar_size={}&rating=PG",
        email_hash, avatar_size
    )
}
