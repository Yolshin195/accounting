use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub size: i64,
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.size
    }
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    100
}