use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::ward::{Ward, CreateWardRequest, UpdateWardRequest};
use crate::domain::repositories::ward_repository::WardRepository;
use crate::domain::value_objects::{WardCode, WardCodeError};
use crate::errors::AppResult;

/// PostgreSQL implementation of WardRepository
#[derive(Clone)]
pub struct PostgresWardRepository {
    pool: PgPool,
}

impl PostgresWardRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WardRepository for PostgresWardRepository {
    async fn create(&self, request: &CreateWardRequest, lga_id: Uuid) -> AppResult<Ward> {
        let ward_code = WardCode::new(request.code.clone())
            .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

        let ward = Ward::new(lga_id, request.name.clone(), ward_code);

        sqlx::query(
            "INSERT INTO wards (id, lga_id, name, code, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&ward.id)
        .bind(&ward.lga_id)
        .bind(&ward.name)
        .bind(ward.code.to_string())
        .bind(&ward.created_at)
        .bind(&ward.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(ward)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Ward>> {
        let row = sqlx::query(
            "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE id = $1"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let ward_code = WardCode::new(row.get::<String, _>("code"))
                    .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(Ward {
                    id: row.get("id"),
                    lga_id: row.get("lga_id"),
                    name: row.get("name"),
                    code: ward_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<Ward>> {
        let row = sqlx::query(
            "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let ward_code = WardCode::new(row.get::<String, _>("code"))
                    .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(Ward {
                    id: row.get("id"),
                    lga_id: row.get("lga_id"),
                    name: row.get("name"),
                    code: ward_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_code(&self, code: &WardCode) -> AppResult<Option<Ward>> {
        let row = sqlx::query(
            "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE code = $1"
        )
        .bind(code.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let ward_code = WardCode::new(row.get::<String, _>("code"))
                    .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(Ward {
                    id: row.get("id"),
                    lga_id: row.get("lga_id"),
                    name: row.get("name"),
                    code: ward_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_lga_id(&self, lga_id: Uuid, page: u32, limit: u32) -> AppResult<Vec<Ward>> {
        let offset = (page - 1) * limit;
        
        let rows = sqlx::query(
            "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE lga_id = $1 ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(&lga_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut wards = Vec::new();
        for row in rows {
            let ward_code = WardCode::new(row.get::<String, _>("code"))
                .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            
            wards.push(Ward {
                id: row.get("id"),
                lga_id: row.get("lga_id"),
                name: row.get("name"),
                code: ward_code,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(wards)
    }

    async fn update(&self, id: Uuid, request: &UpdateWardRequest) -> AppResult<Ward> {
        let mut ward = self.find_by_id(id).await?
            .ok_or_else(|| crate::errors::AppError::NotFound("Ward not found".to_string()))?;

        if let Some(name) = &request.name {
            ward.update_name(name.clone());
        }

        if let Some(code) = &request.code {
            let ward_code = WardCode::new(code.clone())
                .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            ward.update_code(ward_code);
        }

        sqlx::query(
            "UPDATE wards SET name = $1, code = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(&ward.name)
        .bind(ward.code.to_string())
        .bind(&ward.updated_at)
        .bind(&ward.id)
        .execute(&self.pool)
        .await?;

        Ok(ward)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM wards WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound("Ward not found".to_string()));
        }

        Ok(())
    }

    async fn count_by_lga(&self, lga_id: Uuid) -> AppResult<u64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM wards WHERE lga_id = $1")
            .bind(&lga_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count") as u64)
    }

    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<Ward>> {
        let offset = (page - 1) * limit;
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            "SELECT id, lga_id, name, code, created_at, updated_at FROM wards WHERE name ILIKE $1 OR code ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(&search_pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut wards = Vec::new();
        for row in rows {
            let ward_code = WardCode::new(row.get::<String, _>("code"))
                .map_err(|e: WardCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            
            wards.push(Ward {
                id: row.get("id"),
                lga_id: row.get("lga_id"),
                name: row.get("name"),
                code: ward_code,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(wards)
    }
}