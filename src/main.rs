use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use utoipa_swagger_ui::SwaggerUi;

use nigeria_geo_api::{
    api::usage_analytics::{
        cleanup_old_records_handler, get_hourly_stats_handler, get_status_code_stats_handler,
        get_top_endpoints_handler, get_usage_by_api_key_handler, get_usage_by_ip_handler,
        get_usage_stats_handler, refresh_stats_views_handler,
    },
    config::Config,
    infrastructure::repositories::{
        address_repository_impl::PostgresAddressRepository,
        api_usage_repository_impl::PostgresApiUsageRepository,
        lga_repository_impl::PostgresLgaRepository,
        postal_code_repository_impl::PostgresPostalCodeRepository,
        state_repository_impl::PostgresStateRepository,
        ward_repository_impl::PostgresWardRepository,
    },
    presentation::{
        handlers::health_check_handler,
        handlers_simple::{
            find_address_by_components_handler, find_nearby_postal_codes_handler,
            find_similar_addresses_handler, get_lga_by_id_handler, get_lgas_by_state_handler,
            get_postal_code_by_code_handler, get_postal_code_by_id_handler,
            get_postal_codes_by_ward_handler, get_state_by_id_handler, get_states_handler,
            get_ward_by_id_handler, get_wards_by_lga_handler, search_all_handler,
            search_lgas_handler, search_postal_codes_handler, search_states_handler,
            search_wards_handler, validate_address_handler,
        },
        // middleware::usage_tracking::track_usage_middleware,
        state::AppState,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nigeria_geo_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");

    info!("Starting Nigeria Geo API Server");
    info!(
        "Server will run on {}:{}",
        config.server_host, config.server_port
    );

    // Create database connection pool with optimized settings
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .idle_timeout(std::time::Duration::from_secs(300))
        .connect(&config.database_url)
        .await?;

    // Run migrations (temporarily commented out since tables already exist)
    // sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database connection established");

    // Initialize Redis cache with graceful fallback
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let cache = match nigeria_geo_api::infrastructure::cache::CacheClient::new(&redis_url) {
        Ok(client) => {
            info!("✅ Redis cache connected successfully at {}", redis_url);
            client
        }
        Err(e) => {
            warn!(
                "⚠️  Redis connection failed: {}. API will run without caching.",
                e
            );
            info!("To fix this, ensure Redis is running at: {}", redis_url);
            // For now, we'll still create a client but it will fail on operations
            // In a real production setup, you might want to implement a NoOpCacheClient
            nigeria_geo_api::infrastructure::cache::CacheClient::new("redis://127.0.0.1:6379")
                .unwrap_or_else(|_| panic!("Critical: Cannot create cache client"))
        }
    };

    // Initialize repositories
    let state_repository = PostgresStateRepository::new(pool.clone());
    let lga_repository = PostgresLgaRepository::new(pool.clone());
    let ward_repository = PostgresWardRepository::new(pool.clone());
    let postal_code_repository = PostgresPostalCodeRepository::new(pool.clone());
    let address_repository = PostgresAddressRepository::new(
        pool.clone(),
        Box::new(state_repository.clone()),
        Box::new(lga_repository.clone()),
        Box::new(ward_repository.clone()),
        Box::new(postal_code_repository.clone()),
    );

    // Initialize API usage repository
    let api_usage_repository = PostgresApiUsageRepository::new(pool.clone());

    // Initialize unified application state without caching
    let app_state = AppState::new(
        state_repository,
        lga_repository,
        ward_repository,
        postal_code_repository,
        address_repository,
        api_usage_repository,
        pool.clone(),
    );

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the application router with all endpoints
    let app = Router::new()
        // Health endpoint
        .route("/api/v1/health", get(health_check_handler))
        // States endpoints
        .route("/api/v1/states", get(get_states_handler))
        .route("/api/v1/states/:id", get(get_state_by_id_handler))
        .route("/api/v1/states/:id/lgas", get(get_lgas_by_state_handler))
        // LGAs endpoints
        .route("/api/v1/lgas/:id", get(get_lga_by_id_handler))
        .route("/api/v1/lgas/:id/wards", get(get_wards_by_lga_handler))
        // Wards endpoints
        .route("/api/v1/wards/:id", get(get_ward_by_id_handler))
        .route(
            "/api/v1/wards/:id/postal-codes",
            get(get_postal_codes_by_ward_handler),
        )
        // Postal codes endpoints
        .route(
            "/api/v1/postal-codes/:id",
            get(get_postal_code_by_id_handler),
        )
        .route(
            "/api/v1/postal-codes/code/:code",
            get(get_postal_code_by_code_handler),
        )
        .route(
            "/api/v1/postal-codes/nearby",
            get(find_nearby_postal_codes_handler),
        )
        // Address validation endpoints
        .route("/api/v1/validate", post(validate_address_handler))
        .route(
            "/api/v1/address/find",
            get(find_address_by_components_handler),
        )
        .route(
            "/api/v1/address/similar",
            post(find_similar_addresses_handler),
        )
        // Search endpoints
        .route("/api/v1/search", get(search_all_handler))
        .route("/api/v1/search/states", get(search_states_handler))
        .route("/api/v1/search/lgas", get(search_lgas_handler))
        .route("/api/v1/search/wards", get(search_wards_handler))
        .route(
            "/api/v1/search/postal-codes",
            get(search_postal_codes_handler),
        )
        // API Usage Analytics endpoints
        .route(
            "/api/v1/analytics/usage-stats",
            get(get_usage_stats_handler),
        )
        .route(
            "/api/v1/analytics/top-endpoints",
            get(get_top_endpoints_handler),
        )
        .route(
            "/api/v1/analytics/hourly-stats",
            get(get_hourly_stats_handler),
        )
        .route(
            "/api/v1/analytics/status-codes",
            get(get_status_code_stats_handler),
        )
        .route(
            "/api/v1/analytics/usage-by-ip",
            get(get_usage_by_ip_handler),
        )
        .route(
            "/api/v1/analytics/usage-by-api-key",
            get(get_usage_by_api_key_handler),
        )
        .route(
            "/api/v1/analytics/refresh-stats",
            post(refresh_stats_views_handler),
        )
        .route(
            "/api/v1/analytics/cleanup",
            post(cleanup_old_records_handler),
        )
        // OpenAPI documentation (temporarily disabled)
        // .route("/api-docs/openapi.json", get(openapi_json_handler))
        // .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .layer(axum::middleware::from_fn_with_state(
        //     app_state.clone(),
        //     track_usage_middleware,
        // ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Start the server
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await
            .expect("Failed to bind to address");

    info!(
        "🚀 Server running on http://{}:{}",
        config.server_host, config.server_port
    );
    // info!("📊 API Documentation available at http://{}:{}/docs", config.server_host, config.server_port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
