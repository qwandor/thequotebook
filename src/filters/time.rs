use askama::{filter_fn, Values};
use sqlx::types::chrono::{DateTime, Utc};

#[filter_fn]
pub fn long_datetime(datetime: &DateTime<Utc>, _values: &dyn Values) -> askama::Result<String> {
    Ok(datetime.format("%A %d %B %Y at %I:%M %P %Z").to_string())
}
