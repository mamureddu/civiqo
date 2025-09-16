pub mod auth;
pub mod crypto;
pub mod database;
pub mod error;
pub mod models;
pub mod utils;

#[cfg(test)]
pub mod testing;

// Re-export commonly used types
pub use error::{AppError, Result};
pub use models::*;