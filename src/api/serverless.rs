//! Serverless compatibility layer for platforms like Vercel

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::{core_handlers, domains, prompts};
use crate::api::file_streaming::JsonStreamManager;

/// Serverless API state
#[derive(Clone)]
pub struct ServerlessState {
    pub json_manager: std::sync::Arc<JsonStreamManager>,
}

/// Create serverless router
pub fn create_serverless_router() -> Router {
    let json_manager = std::sync::Arc::new(JsonStreamManager::new());
    let state = ServerlessState { json_manager };

    Router::new()
        .route("/health", get(health_check))
        .route("/api/ollama/process", post(serverless_ollama_process))
        .route("/api/available-files", get(list_available_files))
        .with_state(state)
}

/// Health check for serverless
pub async fn health_check() -> Json<Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "ai-json-analysis-api",
        "mode": "serverless"
    }))
}

/// Simplified Ollama processing for serverless
pub async fn serverless_ollama_process(
    State(_state): State<ServerlessState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, StatusCode> {
    // Extract parameters
    let file_path = payload.get("file_path")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let prompt = payload.get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or("Analyze this data and provide insights");
    
    let model = payload.get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("llama2");

    // Simple processing without file watching (serverless limitation)
    let result = process_json_data(file_path, prompt, model).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

/// Process JSON data (serverless version)
async fn process_json_data(
    file_path: &str,
    prompt: &str,
    model: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Read file content
    let file_content = std::fs::read_to_string(file_path)?;
    
    // Create enhanced prompt
    let enhanced_prompt = format!(
        "{}\n\nData: {}",
        prompt,
        serde_json::to_string_pretty(&file_content)?
    );

    // For serverless, we'll return a mock response
    // In production, you'd integrate with your AI service
    Ok(serde_json::json!({
        "status": "success",
        "file_path": file_path,
        "prompt": prompt,
        "model": model,
        "analysis": "Serverless analysis completed - AI integration needed",
        "data_processed": file_content.len(),
        "mode": "serverless"
    }))
}

/// List available files (serverless version)
pub async fn list_available_files() -> Json<Value> {
    let current_dir = std::env::current_dir().unwrap_or_default();
    let mut json_files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        json_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    
    Json(serde_json::json!({
        "status": "success",
        "current_directory": current_dir.to_string_lossy(),
        "available_json_files": json_files,
        "total_files": json_files.len(),
        "mode": "serverless"
    }))
}
