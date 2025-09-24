use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/nigeria_geo".to_string());

    // Create database connection pool
    let pool = PgPool::connect(&database_url).await?;
    
    println!("Connected to database: {}", database_url);
    
    // Run the seed data SQL
    let seed_sql = include_str!("seed_data.sql");
    
    sqlx::query(seed_sql)
        .execute(&pool)
        .await?;
    
    println!("✅ Seed data inserted successfully!");
    
    // Verify the data
    let state_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM states")
        .fetch_one(&pool)
        .await?;
    
    let lga_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lgas")
        .fetch_one(&pool)
        .await?;
    
    let ward_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM wards")
        .fetch_one(&pool)
        .await?;
    
    let postal_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM postal_codes")
        .fetch_one(&pool)
        .await?;
    
    println!("📊 Database Statistics:");
    println!("   States: {}", state_count.0);
    println!("   LGAs: {}", lga_count.0);
    println!("   Wards: {}", ward_count.0);
    println!("   Postal Codes: {}", postal_count.0);
    
    Ok(())
}
