use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Pagination parameters
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PaginationParams {
    /// Page number (1-based)
    #[schema(example = 1, minimum = 1)]
    pub page: Option<u32>,
    /// Number of items per page
    #[schema(example = 20, minimum = 1, maximum = 100)]
    pub limit: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
        }
    }
}

/// Paginated response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T: for<'a> ToSchema<'a>> {
    /// Array of data items
    pub data: Vec<T>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    /// Current page number
    #[schema(example = 1)]
    pub page: u32,
    /// Items per page
    #[schema(example = 20)]
    pub limit: u32,
    /// Total number of items
    #[schema(example = 100)]
    pub total: u64,
    /// Total number of pages
    #[schema(example = 5)]
    pub total_pages: u32,
    /// Whether there is a next page
    #[schema(example = true)]
    pub has_next: bool,
    /// Whether there is a previous page
    #[schema(example = false)]
    pub has_prev: bool,
}

impl<T: for<'a> ToSchema<'a>> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, limit: u32, total: u64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as u32;
        
        Self {
            data,
            pagination: PaginationMeta {
                page,
                limit,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        }
    }
}
