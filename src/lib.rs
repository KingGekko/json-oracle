//! Library crate for AI JSON Analysis API

pub mod api;
pub mod ollama;

// Re-export main functionality
pub use api::start_api_server;

// Serverless handler for Vercel
#[cfg(feature = "serverless")]
pub use api::serverless::create_serverless_router;
