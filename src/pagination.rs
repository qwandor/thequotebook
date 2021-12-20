use paginate::{Page, Pages};
use serde::Deserialize;

/// Query parameter for pagination.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct QueryPage {
    #[serde(default)]
    pub page: usize,
}

pub struct PaginationState {
    pub pages: Pages,
    pub current_page: Page,
}
