use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// State model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct State {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// LGA (Local Government Area) model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Lga {
    pub id: Uuid,
    pub state_id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ward model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ward {
    pub id: Uuid,
    pub lga_id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Postal Code model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostalCode {
    pub id: Uuid,
    pub ward_id: Uuid,
    pub postal_code: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub urban: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// State with LGAs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateWithLgas {
    #[serde(flatten)]
    pub state: State,
    pub lgas: Vec<Lga>,
}

/// LGA with Wards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LgaWithWards {
    #[serde(flatten)]
    pub lga: Lga,
    pub wards: Vec<Ward>,
}

/// Ward with Postal Codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WardWithPostalCodes {
    #[serde(flatten)]
    pub ward: Ward,
    pub postal_codes: Vec<PostalCode>,
}

/// Address validation request
#[derive(Debug, Deserialize, Validate)]
pub struct AddressValidationRequest {
    #[validate(length(min = 1))]
    pub state: String,
    #[validate(length(min = 1))]
    pub lga: String,
    #[validate(length(min = 1))]
    pub ward: String,
    #[validate(length(min = 1))]
    pub postal_code: String,
}

/// Address validation response
#[derive(Debug, Serialize)]
pub struct AddressValidationResponse {
    pub valid: bool,
    pub canonical: Option<CanonicalAddress>,
    pub suggestions: Vec<AddressSuggestion>,
}

/// Canonical address format
#[derive(Debug, Serialize)]
pub struct CanonicalAddress {
    pub state: State,
    pub lga: Lga,
    pub ward: Ward,
    pub postal_code: PostalCode,
}

/// Address suggestion
#[derive(Debug, Serialize)]
pub struct AddressSuggestion {
    pub state: Option<State>,
    pub lga: Option<Lga>,
    pub ward: Option<Ward>,
    pub postal_code: Option<PostalCode>,
    pub reason: String,
}

/// Search result
#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub states: Vec<State>,
    pub lgas: Vec<Lga>,
    pub wards: Vec<Ward>,
    pub postal_codes: Vec<PostalCode>,
}

/// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
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
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}
