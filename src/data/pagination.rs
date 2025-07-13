use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 50,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub pages: usize,
    pub time_ms: u128,
}

impl<T> PagedResult<T> {
    pub fn new(
        items: Vec<T>,
        total: usize,
        pagination: &Pagination,
        time: std::time::Duration,
    ) -> Self {
        let pages = if pagination.per_page > 0 {
            (total + pagination.per_page - 1) / pagination.per_page
        } else {
            0
        };

        Self {
            items,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            pages,
            time_ms: time.as_millis(),
        }
    }
}
