pub mod application;
pub mod config;
pub mod domain;
pub mod errors;
pub mod infrastructure;
pub mod presentation;

// Re-exports for convenience
pub use config::Config;
pub use errors::{AppError, AppResult};
