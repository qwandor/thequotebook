use paginate::{Page, Pages};
use serde::Deserialize;
use std::cmp::min;

/// Query parameter for pagination.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct QueryPage {
    #[serde(default)]
    pub page: usize,
}

pub struct PaginationState {
    pub pages: Pages,
    pub current_page: Page,
    /// The number of pages to include either side of the current page.
    pub window_size: usize,
}

impl PaginationState {
    pub fn page_links(&self) -> Vec<PageOrGap> {
        let mut links = vec![];

        if self.current_page.offset > self.window_size {
            // There's a gap before the window.
            links.push(PageOrGap::Page(self.pages.with_offset(0)));
            // Only insert the gap marker if there is actually a gap of at least one page.
            if self.current_page.offset > self.window_size + 1 {
                links.push(PageOrGap::Gap);
            }
        }

        // Pages before the current page.
        for offset in
            self.current_page.offset.saturating_sub(self.window_size)..self.current_page.offset
        {
            links.push(PageOrGap::Page(self.pages.with_offset(offset)));
        }

        // Current page.
        links.push(PageOrGap::CurrentPage(self.current_page.clone()));

        // Pages after the current page.
        for offset in self.current_page.offset + 1
            ..min(
                self.current_page.offset + self.window_size + 1,
                self.pages.page_count(),
            )
        {
            links.push(PageOrGap::Page(self.pages.with_offset(offset)));
        }

        if self.current_page.offset + self.window_size + 1 < self.pages.page_count() {
            // There's a gap after the window.
            // Only insert the gap marker if there is actually a gap of at least one page.
            if self.current_page.offset + self.window_size + 2 < self.pages.page_count() {
                links.push(PageOrGap::Gap);
            }
            links.push(PageOrGap::Page(
                self.pages.with_offset(self.pages.page_count() - 1),
            ));
        }

        links
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PageOrGap {
    Page(Page),
    CurrentPage(Page),
    Gap,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_page() {
        let pages = Pages::new(7, 10);
        let current_page = pages.with_offset(0);
        let state = PaginationState {
            pages,
            current_page: current_page.clone(),
            window_size: 2,
        };
        assert_eq!(
            state.page_links(),
            vec![PageOrGap::CurrentPage(current_page)]
        );
    }

    #[test]
    fn no_gaps() {
        let pages = Pages::new(50, 10);
        let current_page = pages.with_offset(2);
        let state = PaginationState {
            pages: pages.clone(),
            current_page: current_page.clone(),
            window_size: 2,
        };
        assert_eq!(
            state.page_links(),
            vec![
                PageOrGap::Page(pages.with_offset(0)),
                PageOrGap::Page(pages.with_offset(1)),
                PageOrGap::CurrentPage(state.current_page),
                PageOrGap::Page(pages.with_offset(3)),
                PageOrGap::Page(pages.with_offset(4)),
            ]
        );
    }

    #[test]
    fn gaps() {
        let pages = Pages::new(100, 10);
        let current_page = pages.with_offset(5);
        let state = PaginationState {
            pages: pages.clone(),
            current_page: current_page.clone(),
            window_size: 2,
        };
        assert_eq!(
            state.page_links(),
            vec![
                PageOrGap::Page(pages.with_offset(0)),
                PageOrGap::Gap,
                PageOrGap::Page(pages.with_offset(3)),
                PageOrGap::Page(pages.with_offset(4)),
                PageOrGap::CurrentPage(state.current_page),
                PageOrGap::Page(pages.with_offset(6)),
                PageOrGap::Page(pages.with_offset(7)),
                PageOrGap::Gap,
                PageOrGap::Page(pages.with_offset(9)),
            ]
        );
    }

    #[test]
    fn almost_gaps() {
        let pages = Pages::new(70, 10);
        let current_page = pages.with_offset(3);
        let state = PaginationState {
            pages: pages.clone(),
            current_page: current_page.clone(),
            window_size: 2,
        };
        assert_eq!(
            state.page_links(),
            vec![
                PageOrGap::Page(pages.with_offset(0)),
                PageOrGap::Page(pages.with_offset(1)),
                PageOrGap::Page(pages.with_offset(2)),
                PageOrGap::CurrentPage(state.current_page),
                PageOrGap::Page(pages.with_offset(4)),
                PageOrGap::Page(pages.with_offset(5)),
                PageOrGap::Page(pages.with_offset(6)),
            ]
        );
    }
}
