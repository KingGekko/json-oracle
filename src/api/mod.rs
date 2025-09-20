//! Core API module for AI-powered JSON analysis
//! Provides REST endpoints and WebSocket support for streaming JSON data

pub mod file_streaming;
pub mod api_server;
pub mod core_handlers;
pub mod domains;
pub mod prompts;

pub use api_server::start_api_server; 