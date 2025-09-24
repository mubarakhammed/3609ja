use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::*;
use crate::errors::AppResult;

/// Get all states with pagination
pub async fn get_states(
    pool: &PgPool,
    page: u32,
    limit: u32,
) -> AppResult<PaginatedResponse<State>> {
    let offset = (page - 1) * limit;
    
    let states = sqlx::query_as::<_, State>(
        "SELECT id, name, code, created_at, updated_at FROM states ORDER BY name LIMIT $1 OFFSET $2"
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM states")
        .fetch_one(pool)
        .await?;

    let total_pages = (total.0 as f64 / limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        data: states,
        pagination: PaginationMeta {
            page,
            limit,
            total: total.0 as u64,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        },
    })
}

/// Get LGAs by state ID
pub async fn get_lgas_by_state(
    pool: &PgPool,
    state_id: Uuid,
    page: u32,
    limit: u32,
) -> AppResult<PaginatedResponse<Lga>> {
    let offset = (page - 1) * limit;
    
    let lgas = sqlx::query_as::<_, Lga>(
        "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE state_id = $1 ORDER BY name LIMIT $2 OFFSET $3"
    )
    .bind(state_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lgas WHERE state_id = $1")
        .bind(state_id)
        .fetch_one(pool)
        .await?;

    let total_pages = (total.0 as f64 / limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        data: lgas,
        pagination: PaginationMeta {
            page,
            limit,
            total: total.0 as u64,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        },
    })
}

/// Get wards by LGA ID
pub async fn get_wards_by_lga(
    pool: &PgPool,
    lga_id: Uuid,
    page: u32,
    limit: u32,
) -> AppResult<PaginatedResponse<Ward>> {
    let offset = (page - 1) * limit;
    
    let wards = sqlx::query_as::<_, Ward>(
        "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE lga_id = $1 ORDER BY name LIMIT $2 OFFSET $3"
    )
    .bind(lga_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM wards WHERE lga_id = $1")
        .bind(lga_id)
        .fetch_one(pool)
        .await?;

    let total_pages = (total.0 as f64 / limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        data: wards,
        pagination: PaginationMeta {
            page,
            limit,
            total: total.0 as u64,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        },
    })
}

/// Get postal codes by ward ID
pub async fn get_postal_codes_by_ward(
    pool: &PgPool,
    ward_id: Uuid,
    page: u32,
    limit: u32,
) -> AppResult<PaginatedResponse<PostalCode>> {
    let offset = (page - 1) * limit;
    
    let postal_codes = sqlx::query_as::<_, PostalCode>(
        "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE ward_id = $1 ORDER BY postal_code LIMIT $2 OFFSET $3"
    )
    .bind(ward_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM postal_codes WHERE ward_id = $1")
        .bind(ward_id)
        .fetch_one(pool)
        .await?;

    let total_pages = (total.0 as f64 / limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        data: postal_codes,
        pagination: PaginationMeta {
            page,
            limit,
            total: total.0 as u64,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        },
    })
}

/// Search across all entities
pub async fn search_all(
    pool: &PgPool,
    query: &str,
    page: u32,
    limit: u32,
) -> AppResult<SearchResult> {
    let offset = (page - 1) * limit;
    let search_pattern = format!("%{}%", query);

    // Search states
    let states = sqlx::query_as::<_, State>(
        "SELECT id, name, code, created_at, updated_at FROM states WHERE name ILIKE $1 OR code ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
    )
    .bind(&search_pattern)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    // Search LGAs
    let lgas = sqlx::query_as::<_, Lga>(
        "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE name ILIKE $1 OR code ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
    )
    .bind(&search_pattern)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    // Search wards
    let wards = sqlx::query_as::<_, Ward>(
        "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE name ILIKE $1 OR code ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
    )
    .bind(&search_pattern)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    // Search postal codes
    let postal_codes = sqlx::query_as::<_, PostalCode>(
        "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE postal_code ILIKE $1 ORDER BY postal_code LIMIT $2 OFFSET $3"
    )
    .bind(&search_pattern)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    Ok(SearchResult {
        states,
        lgas,
        wards,
        postal_codes,
    })
}

/// Validation result row
#[derive(sqlx::FromRow)]
struct ValidationRow {
    state_id: Uuid,
    state_name: String,
    state_code: String,
    lga_id: Uuid,
    lga_name: String,
    lga_code: String,
    ward_id: Uuid,
    ward_name: String,
    ward_code: String,
    postal_id: Uuid,
    postal_code: String,
    lat: Option<f64>,
    lng: Option<f64>,
    urban: bool,
}

/// Validate address
pub async fn validate_address(
    pool: &PgPool,
    request: &AddressValidationRequest,
) -> AppResult<AddressValidationResponse> {
    // Find exact match
    let exact_match = sqlx::query_as::<_, ValidationRow>(
        r#"
        SELECT 
            s.id as state_id, s.name as state_name, s.code as state_code,
            l.id as lga_id, l.name as lga_name, l.code as lga_code,
            w.id as ward_id, w.name as ward_name, w.code as ward_code,
            p.id as postal_id, p.postal_code, p.lat, p.lng, p.urban
        FROM states s
        JOIN lgas l ON s.id = l.state_id
        JOIN wards w ON l.id = w.lga_id
        JOIN postal_codes p ON w.id = p.ward_id
        WHERE s.name ILIKE $1 
        AND l.name ILIKE $2 
        AND w.name ILIKE $3 
        AND p.postal_code = $4
        "#
    )
    .bind(&request.state)
    .bind(&request.lga)
    .bind(&request.ward)
    .bind(&request.postal_code)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = exact_match {
        // Exact match found
        let canonical = CanonicalAddress {
            state: State {
                id: row.state_id,
                name: row.state_name,
                code: row.state_code,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            lga: Lga {
                id: row.lga_id,
                state_id: row.state_id,
                name: row.lga_name,
                code: row.lga_code,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            ward: Ward {
                id: row.ward_id,
                lga_id: row.lga_id,
                name: row.ward_name,
                code: row.ward_code,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            postal_code: PostalCode {
                id: row.postal_id,
                ward_id: row.ward_id,
                postal_code: row.postal_code,
                lat: row.lat,
                lng: row.lng,
                urban: row.urban,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        };

        return Ok(AddressValidationResponse {
            valid: true,
            canonical: Some(canonical),
            suggestions: vec![],
        });
    }

    // No exact match, find suggestions
    let mut suggestions = Vec::new();

    // Find similar states
    let similar_states = sqlx::query_as::<_, State>(
        "SELECT id, name, code, created_at, updated_at FROM states WHERE name ILIKE $1 LIMIT 3"
    )
    .bind(format!("%{}%", request.state))
    .fetch_all(pool)
    .await?;

    for state in similar_states {
        suggestions.push(AddressSuggestion {
            state: Some(state),
            lga: None,
            ward: None,
            postal_code: None,
            reason: "Similar state found".to_string(),
        });
    }

    // Find similar LGAs
    let similar_lgas = sqlx::query_as::<_, Lga>(
        "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE name ILIKE $1 LIMIT 3"
    )
    .bind(format!("%{}%", request.lga))
    .fetch_all(pool)
    .await?;

    for lga in similar_lgas {
        suggestions.push(AddressSuggestion {
            state: None,
            lga: Some(lga),
            ward: None,
            postal_code: None,
            reason: "Similar LGA found".to_string(),
        });
    }

    // Find similar wards
    let similar_wards = sqlx::query_as::<_, Ward>(
        "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE name ILIKE $1 LIMIT 3"
    )
    .bind(format!("%{}%", request.ward))
    .fetch_all(pool)
    .await?;

    for ward in similar_wards {
        suggestions.push(AddressSuggestion {
            state: None,
            lga: None,
            ward: Some(ward),
            postal_code: None,
            reason: "Similar ward found".to_string(),
        });
    }

    // Find similar postal codes
    let similar_postal_codes = sqlx::query_as::<_, PostalCode>(
        "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE postal_code ILIKE $1 LIMIT 3"
    )
    .bind(format!("%{}%", request.postal_code))
    .fetch_all(pool)
    .await?;

    for postal_code in similar_postal_codes {
        suggestions.push(AddressSuggestion {
            state: None,
            lga: None,
            ward: None,
            postal_code: Some(postal_code),
            reason: "Similar postal code found".to_string(),
        });
    }

    Ok(AddressValidationResponse {
        valid: false,
        canonical: None,
        suggestions,
    })
}