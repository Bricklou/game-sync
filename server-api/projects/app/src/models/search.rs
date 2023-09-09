use sea_orm::Order;
use validator::Validate;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Into<Order> for SortOrder {
    fn into(self) -> Order {
        match self {
            Self::Asc => Order::Asc,
            Self::Desc => Order::Desc,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Validate)]
pub struct Search {
    search: Option<String>,
    sort_order: Option<SortOrder>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            search: None,
            sort_order: Some(SortOrder::Asc),
        }
    }
}

impl Search {
    pub fn get_sort_order(&self) -> SortOrder {
        self.sort_order.clone().unwrap_or(SortOrder::Asc)
    }

    pub fn get_search(&self) -> Option<String> {
        self.search.clone()
    }
}
