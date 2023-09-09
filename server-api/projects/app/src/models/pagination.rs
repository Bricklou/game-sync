use validator::Validate;

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
pub struct Pagination {
    page: Option<u64>,
    per_page: Option<u64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
        }
    }
}

impl Pagination {
    pub fn get_page(&self) -> u64 {
        self.page.unwrap_or(0)
    }

    pub fn get_per_page(&self) -> u64 {
        self.per_page.unwrap_or(10)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginationMeta {
    pub current_page: u64,
    pub total_items: u64,
    pub total_pages: u64,
    pub per_page: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}
