//! Nigeria Geo + Postal + Validation API
//! 
//! A production-ready API for Nigerian geographical data built with Clean Architecture principles.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod errors;
pub mod config;

// Re-exports for convenience
pub use errors::{AppError, AppResult};
pub use config::Config;