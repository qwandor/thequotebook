use sqlx::types::chrono::{DateTime, Utc};

pub fn long_datetime(datetime: &DateTime<Utc>) -> askama::Result<String> {
    Ok(datetime.format("%A %d %B %Y at %I:%M %P %Z").to_string())
}
