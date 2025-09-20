//! User-specific API handlers for authenticated users
//! Provides endpoints for user dashboards, integrations, and analytics

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::auth::{get_current_user, ClerkUser};
use super::integration_manager::{IntegrationManager, CreateIntegrationRequest, Integration, IntegrationAnalysisResult};
use super::core_handlers::ApiState;

/// Create user-specific routes
pub fn create_user_routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/user/integrations", get(get_user_integrations))
        .route("/user/integrations", post(create_user_integration))
        .route("/user/integrations/:id", delete(delete_user_integration))
        .route("/user/integrations/:id/results", get(get_user_integration_results))
        .route("/user/stats", get(get_user_stats))
        .route("/user/profile", get(get_user_profile))
        .route("/user/analytics", get(get_user_analytics))
}

/// Get integrations for the authenticated user
async fn get_user_integrations(
    State(state): State<Arc<ApiState>>,
    request: axum::extract::Request,
) -> Result<Json<Vec<Integration>>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // For now, we'll use a simple integration manager
    // In production, you'd get this from the state
    let manager = IntegrationManager::new();
    let integrations = manager.get_user_integrations(&user.id).await;
    
    Ok(Json(integrations))
}

/// Create a new integration for the authenticated user
async fn create_user_integration(
    State(state): State<Arc<ApiState>>,
    request: axum::extract::Request,
    Json(integration_request): Json<CreateIntegrationRequest>,
) -> Result<Json<Integration>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let manager = IntegrationManager::new();
    match manager.create_user_integration(&user.id, integration_request).await {
        Ok(integration) => Ok(Json(integration)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Delete a user's integration
async fn delete_user_integration(
    State(state): State<Arc<ApiState>>,
    Path(integration_id): Path<String>,
    request: axum::extract::Request,
) -> Result<StatusCode, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let manager = IntegrationManager::new();
    
    // Verify the integration belongs to the user
    if let Some(integration) = manager.get_integration(&integration_id).await {
        if integration.user_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
        
        if manager.delete_integration(&integration_id).await {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get analysis results for a user's integration
async fn get_user_integration_results(
    State(state): State<Arc<ApiState>>,
    Path(integration_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    request: axum::extract::Request,
) -> Result<Json<Vec<IntegrationAnalysisResult>>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let manager = IntegrationManager::new();
    
    // Verify the integration belongs to the user
    if let Some(integration) = manager.get_integration(&integration_id).await {
        if integration.user_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
        
        let limit = params.get("limit").and_then(|l| l.parse().ok());
        let results = manager.get_analysis_results(&integration_id, limit).await;
        Ok(Json(results))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get user dashboard statistics
async fn get_user_stats(
    State(state): State<Arc<ApiState>>,
    request: axum::extract::Request,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let manager = IntegrationManager::new();
    let stats = manager.get_user_dashboard_stats(&user.id).await;
    
    Ok(Json(stats))
}

/// Get user profile information
async fn get_user_profile(
    State(state): State<Arc<ApiState>>,
    request: axum::extract::Request,
) -> Result<Json<UserProfile>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let profile = UserProfile {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        image_url: user.image_url,
        created_at: user.created_at,
        plan: "free".to_string(), // Default plan
        api_calls_this_month: 0,   // Would be calculated from actual usage
        api_calls_limit: 10000,    // Free tier limit
    };

    Ok(Json(profile))
}

/// Get user analytics data
async fn get_user_analytics(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<HashMap<String, String>>,
    request: axum::extract::Request,
) -> Result<Json<UserAnalytics>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Get time range from query params (default to last 30 days)
    let days = params.get("days").and_then(|d| d.parse().ok()).unwrap_or(30);
    
    let manager = IntegrationManager::new();
    let integrations = manager.get_user_integrations(&user.id).await;
    
    // Mock analytics data - in production, this would be calculated from actual usage
    let analytics = UserAnalytics {
        total_api_calls: 156,
        successful_calls: 148,
        failed_calls: 8,
        average_response_time: 2.3,
        most_used_integration: integrations.first().map(|i| i.name.clone()).unwrap_or_default(),
        daily_usage: vec![
            DailyUsage { date: "2024-01-01".to_string(), calls: 5 },
            DailyUsage { date: "2024-01-02".to_string(), calls: 8 },
            DailyUsage { date: "2024-01-03".to_string(), calls: 12 },
            DailyUsage { date: "2024-01-04".to_string(), calls: 7 },
            DailyUsage { date: "2024-01-05".to_string(), calls: 15 },
        ],
        top_domains: vec![
            DomainUsage { domain: "ecommerce".to_string(), calls: 45, percentage: 28.8 },
            DomainUsage { domain: "finance".to_string(), calls: 32, percentage: 20.5 },
            DomainUsage { domain: "healthcare".to_string(), calls: 28, percentage: 17.9 },
        ],
    };

    Ok(Json(analytics))
}

/// User profile response
#[derive(Debug, Serialize)]
struct UserProfile {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    image_url: Option<String>,
    created_at: i64,
    plan: String,
    api_calls_this_month: u32,
    api_calls_limit: u32,
}

/// User analytics response
#[derive(Debug, Serialize)]
struct UserAnalytics {
    total_api_calls: u32,
    successful_calls: u32,
    failed_calls: u32,
    average_response_time: f64,
    most_used_integration: String,
    daily_usage: Vec<DailyUsage>,
    top_domains: Vec<DomainUsage>,
}

#[derive(Debug, Serialize)]
struct DailyUsage {
    date: String,
    calls: u32,
}

#[derive(Debug, Serialize)]
struct DomainUsage {
    domain: String,
    calls: u32,
    percentage: f64,
}
