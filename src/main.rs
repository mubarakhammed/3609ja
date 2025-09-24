use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use nigeria_geo_api::{
    config::Config,
    infrastructure::repositories::state_repository_impl::PostgresStateRepository,
    application::use_cases::state_use_cases::StateUseCases,
    presentation::controllers::state_controller::{StateController, get_states_handler, get_state_by_id_handler},
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
    info!("Server will run on {}:{}", config.server_host, config.server_port);

    // Create database connection pool
    let pool = PgPool::connect(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    info!("Database migrations completed");

    // Initialize repositories
    let state_repository = PostgresStateRepository::new(pool.clone());

    // Initialize use cases
    let state_use_cases = StateUseCases::new(state_repository);

    // Initialize controllers
    let state_controller = StateController::new(state_use_cases);

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the application router
    let app = Router::new()
        .route("/api/v1/states", get(get_states_handler))
        .route("/api/v1/states/:id", get(get_state_by_id_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state_controller);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
        .await
        .expect("Failed to bind to address");

    info!("🚀 Server running on http://{}:{}", config.server_host, config.server_port);
    info!("📊 API Documentation available at http://{}:{}/docs", config.server_host, config.server_port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}