use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub size: i64,
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        self.page * self.size
    }
}

fn default_page() -> i64 {
    0
}

fn default_per_page() -> i64 {
    100
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagedResponse<T> {
    pub content: Vec<T>,
    pub page: PageInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    pub size: i64,
    pub number: i64,
    #[serde(rename = "totalElements")]
    pub total_elements: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

impl<T> PagedResponse<T> {
    pub fn new(content: Vec<T>, pagination: &Pagination, total_elements: i64) -> Self {
        let total_pages = if pagination.size > 0 {
            (total_elements + pagination.size - 1) / pagination.size
        } else {
            0
        };

        Self {
            content,
            page: PageInfo {
                size: pagination.size,
                number: pagination.page,
                total_elements,
                total_pages,
            },
        }
    }
}
