use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::postal_code::{PostalCode, CreatePostalCodeRequest, UpdatePostalCodeRequest};
use crate::domain::repositories::postal_code_repository::PostalCodeRepository;
use crate::domain::value_objects::{PostalCode as PostalCodeValue, PostalCodeError, Coordinates, CoordinatesError};
use crate::errors::AppResult;

/// PostgreSQL implementation of PostalCodeRepository
#[derive(Clone)]
pub struct PostgresPostalCodeRepository {
    pool: PgPool,
}

impl PostgresPostalCodeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostalCodeRepository for PostgresPostalCodeRepository {
    async fn create(&self, request: &CreatePostalCodeRequest, ward_id: Uuid) -> AppResult<PostalCode> {
        let postal_code_value = PostalCodeValue::new(request.postal_code.clone())
            .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

        let coordinates = match (request.lat, request.lng) {
            (Some(lat), Some(lng)) => Some(
                Coordinates::new(lat, lng)
                    .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
            ),
            _ => None,
        };

        let postal_code = PostalCode::new(ward_id, postal_code_value, coordinates, request.urban);

        sqlx::query(
            "INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&postal_code.id)
        .bind(&postal_code.ward_id)
        .bind(postal_code.postal_code.to_string())
        .bind(&postal_code.coordinates.as_ref().map(|c| c.latitude))
        .bind(&postal_code.coordinates.as_ref().map(|c| c.longitude))
        .bind(&postal_code.urban)
        .bind(&postal_code.created_at)
        .bind(&postal_code.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(postal_code)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<PostalCode>> {
        let row = sqlx::query(
            "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE id = $1"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let postal_code_value = PostalCodeValue::new(row.get::<String, _>("postal_code"))
                    .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

                let coordinates = match (row.get::<Option<f64>, _>("lat"), row.get::<Option<f64>, _>("lng")) {
                    (Some(lat), Some(lng)) => Some(
                        Coordinates::new(lat, lng)
                            .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
                    ),
                    _ => None,
                };
                
                Ok(Some(PostalCode {
                    id: row.get("id"),
                    ward_id: row.get("ward_id"),
                    postal_code: postal_code_value,
                    coordinates,
                    urban: row.get("urban"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_code(&self, code: &PostalCodeValue) -> AppResult<Option<PostalCode>> {
        let row = sqlx::query(
            "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE postal_code = $1"
        )
        .bind(code.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let postal_code_value = PostalCodeValue::new(row.get::<String, _>("postal_code"))
                    .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

                let coordinates = match (row.get::<Option<f64>, _>("lat"), row.get::<Option<f64>, _>("lng")) {
                    (Some(lat), Some(lng)) => Some(
                        Coordinates::new(lat, lng)
                            .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
                    ),
                    _ => None,
                };
                
                Ok(Some(PostalCode {
                    id: row.get("id"),
                    ward_id: row.get("ward_id"),
                    postal_code: postal_code_value,
                    coordinates,
                    urban: row.get("urban"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_ward_id(&self, ward_id: Uuid, page: u32, limit: u32) -> AppResult<Vec<PostalCode>> {
        let offset = (page - 1) * limit;
        
        let rows = sqlx::query(
            "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE ward_id = $1 ORDER BY postal_code LIMIT $2 OFFSET $3"
        )
        .bind(&ward_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut postal_codes = Vec::new();
        for row in rows {
            let postal_code_value = PostalCodeValue::new(row.get::<String, _>("postal_code"))
                .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

            let coordinates = match (row.get::<Option<f64>, _>("lat"), row.get::<Option<f64>, _>("lng")) {
                (Some(lat), Some(lng)) => Some(
                    Coordinates::new(lat, lng)
                        .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
                ),
                _ => None,
            };
            
            postal_codes.push(PostalCode {
                id: row.get("id"),
                ward_id: row.get("ward_id"),
                postal_code: postal_code_value,
                coordinates,
                urban: row.get("urban"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(postal_codes)
    }

    async fn find_near_coordinates(&self, coordinates: &Coordinates, radius_km: f64) -> AppResult<Vec<PostalCode>> {
        let rows = sqlx::query(
            r#"
            SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at
            FROM postal_codes 
            WHERE lat IS NOT NULL AND lng IS NOT NULL
            AND (
                6371 * acos(
                    cos(radians($1)) * cos(radians(lat)) * 
                    cos(radians(lng) - radians($2)) + 
                    sin(radians($1)) * sin(radians(lat))
                )
            ) <= $3
            ORDER BY (
                6371 * acos(
                    cos(radians($1)) * cos(radians(lat)) * 
                    cos(radians(lng) - radians($2)) + 
                    sin(radians($1)) * sin(radians(lat))
                )
            )
            LIMIT 50
            "#
        )
        .bind(coordinates.latitude)
        .bind(coordinates.longitude)
        .bind(radius_km)
        .fetch_all(&self.pool)
        .await?;

        let mut postal_codes = Vec::new();
        for row in rows {
            let postal_code_value = PostalCodeValue::new(row.get::<String, _>("postal_code"))
                .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

            let coordinates = match (row.get::<Option<f64>, _>("lat"), row.get::<Option<f64>, _>("lng")) {
                (Some(lat), Some(lng)) => Some(
                    Coordinates::new(lat, lng)
                        .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
                ),
                _ => None,
            };
            
            postal_codes.push(PostalCode {
                id: row.get("id"),
                ward_id: row.get("ward_id"),
                postal_code: postal_code_value,
                coordinates,
                urban: row.get("urban"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(postal_codes)
    }

    async fn update(&self, id: Uuid, request: &UpdatePostalCodeRequest) -> AppResult<PostalCode> {
        let mut postal_code = self.find_by_id(id).await?
            .ok_or_else(|| crate::errors::AppError::NotFound("Postal code not found".to_string()))?;

        if let Some(code) = &request.postal_code {
            let postal_code_value = PostalCodeValue::new(code.clone())
                .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            postal_code.postal_code = postal_code_value;
        }

        if let (Some(lat), Some(lng)) = (request.lat, request.lng) {
            let coordinates = Some(
                Coordinates::new(lat, lng)
                    .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
            );
            postal_code.update_coordinates(coordinates);
        }

        if let Some(urban) = request.urban {
            postal_code.update_urban_status(urban);
        }

        sqlx::query(
            "UPDATE postal_codes SET postal_code = $1, lat = $2, lng = $3, urban = $4, updated_at = $5 WHERE id = $6"
        )
        .bind(postal_code.postal_code.to_string())
        .bind(&postal_code.coordinates.as_ref().map(|c| c.latitude))
        .bind(&postal_code.coordinates.as_ref().map(|c| c.longitude))
        .bind(&postal_code.urban)
        .bind(&postal_code.updated_at)
        .bind(&postal_code.id)
        .execute(&self.pool)
        .await?;

        Ok(postal_code)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM postal_codes WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound("Postal code not found".to_string()));
        }

        Ok(())
    }

    async fn count_by_ward(&self, ward_id: Uuid) -> AppResult<u64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM postal_codes WHERE ward_id = $1")
            .bind(&ward_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count") as u64)
    }

    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<PostalCode>> {
        let offset = (page - 1) * limit;
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            "SELECT id, ward_id, postal_code, lat, lng, urban, created_at, updated_at FROM postal_codes WHERE postal_code ILIKE $1 ORDER BY postal_code LIMIT $2 OFFSET $3"
        )
        .bind(&search_pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut postal_codes = Vec::new();
        for row in rows {
            let postal_code_value = PostalCodeValue::new(row.get::<String, _>("postal_code"))
                .map_err(|e: PostalCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

            let coordinates = match (row.get::<Option<f64>, _>("lat"), row.get::<Option<f64>, _>("lng")) {
                (Some(lat), Some(lng)) => Some(
                    Coordinates::new(lat, lng)
                        .map_err(|e: CoordinatesError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?
                ),
                _ => None,
            };
            
            postal_codes.push(PostalCode {
                id: row.get("id"),
                ward_id: row.get("ward_id"),
                postal_code: postal_code_value,
                coordinates,
                urban: row.get("urban"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(postal_codes)
    }
}