//! Integration Manager for external system connections
//! Allows users to integrate JSON Oracle API into their systems and monitor results

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Integration configuration for external systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub id: String,
    pub name: String,
    pub system_type: SystemType,
    pub api_key: String,
    pub webhook_url: Option<String>,
    pub status: IntegrationStatus,
    pub created_at: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
    pub configuration: IntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    Webhook,
    RestApi,
    Database,
    FileSystem,
    MessageQueue,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Active,
    Inactive,
    Error,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub auto_analyze: bool,
    pub analysis_domain: Option<String>,
    pub ai_model: Option<String>,
    pub notification_settings: NotificationSettings,
    pub data_filters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub webhook_notifications: bool,
    pub dashboard_alerts: bool,
    pub real_time_updates: bool,
}

/// Analysis result from external system integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAnalysisResult {
    pub id: String,
    pub integration_id: String,
    pub system_name: String,
    pub data_source: String,
    pub analysis_result: serde_json::Value,
    pub status: AnalysisStatus,
    pub created_at: DateTime<Utc>,
    pub processing_time: f64,
    pub insights_count: usize,
    pub recommendations_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisStatus {
    Processing,
    Completed,
    Failed,
    Pending,
}

/// Request to create a new integration
#[derive(Debug, Deserialize)]
pub struct CreateIntegrationRequest {
    pub name: String,
    pub system_type: SystemType,
    pub webhook_url: Option<String>,
    pub configuration: IntegrationConfig,
}

/// Request to send data for analysis
#[derive(Debug, Deserialize)]
pub struct AnalysisRequest {
    pub integration_id: String,
    pub api_key: String,
    pub data: serde_json::Value,
    pub domain: Option<String>,
    pub model: Option<String>,
    pub callback_url: Option<String>,
}

/// Integration Manager state
#[derive(Debug, Clone)]
pub struct IntegrationManager {
    integrations: Arc<RwLock<HashMap<String, Integration>>>,
    analysis_results: Arc<RwLock<HashMap<String, Vec<IntegrationAnalysisResult>>>>,
}

impl IntegrationManager {
    pub fn new() -> Self {
        Self {
            integrations: Arc::new(RwLock::new(HashMap::new())),
            analysis_results: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new integration
    pub async fn create_integration(&self, request: CreateIntegrationRequest) -> Result<Integration, String> {
        let integration_id = Uuid::new_v4().to_string();
        let api_key = format!("json_oracle_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        let integration = Integration {
            id: integration_id.clone(),
            name: request.name,
            system_type: request.system_type,
            api_key,
            webhook_url: request.webhook_url,
            status: IntegrationStatus::Active,
            created_at: Utc::now(),
            last_activity: None,
            configuration: request.configuration,
        };

        let mut integrations = self.integrations.write().await;
        integrations.insert(integration_id.clone(), integration.clone());
        
        // Initialize analysis results for this integration
        let mut results = self.analysis_results.write().await;
        results.insert(integration_id, Vec::new());

        Ok(integration)
    }

    /// Get integration by ID
    pub async fn get_integration(&self, id: &str) -> Option<Integration> {
        let integrations = self.integrations.read().await;
        integrations.get(id).cloned()
    }

    /// Get integration by API key
    pub async fn get_integration_by_api_key(&self, api_key: &str) -> Option<Integration> {
        let integrations = self.integrations.read().await;
        integrations.values().find(|i| i.api_key == api_key).cloned()
    }

    /// List all integrations
    pub async fn list_integrations(&self) -> Vec<Integration> {
        let integrations = self.integrations.read().await;
        integrations.values().cloned().collect()
    }

    /// Delete integration
    pub async fn delete_integration(&self, id: &str) -> bool {
        let mut integrations = self.integrations.write().await;
        let mut results = self.analysis_results.write().await;
        
        integrations.remove(id);
        results.remove(id);
        
        true
    }

    /// Process analysis request from external system
    pub async fn process_analysis_request(
        &self,
        request: AnalysisRequest,
        ollama_client: &crate::ollama::OllamaClient,
    ) -> Result<IntegrationAnalysisResult, String> {
        // Validate integration
        let integration = self.get_integration_by_api_key(&request.api_key).await
            .ok_or("Invalid API key")?;

        if matches!(integration.status, IntegrationStatus::Inactive) {
            return Err("Integration is inactive".to_string());
        }

        let result_id = Uuid::new_v4().to_string();
        let start_time = std::time::Instant::now();

        // Create analysis result record
        let mut analysis_result = IntegrationAnalysisResult {
            id: result_id.clone(),
            integration_id: integration.id.clone(),
            system_name: integration.name.clone(),
            data_source: "external_system".to_string(),
            analysis_result: serde_json::Value::Null,
            status: AnalysisStatus::Processing,
            created_at: Utc::now(),
            processing_time: 0.0,
            insights_count: 0,
            recommendations_count: 0,
        };

        // Store the processing result
        {
            let mut results = self.analysis_results.write().await;
            if let Some(integration_results) = results.get_mut(&integration.id) {
                integration_results.push(analysis_result.clone());
            }
        }

        // Perform AI analysis
        let domain = request.domain.unwrap_or_else(|| "generic".to_string());
        let model = request.model.unwrap_or_else(|| "llama2".to_string());
        
        let prompt = format!(
            "Analyze this {} data from external system '{}' and provide comprehensive insights:",
            domain,
            integration.name
        );

        match ollama_client.generate_optimized(&model, &prompt).await {
            Ok(ai_response) => {
                let processing_time = start_time.elapsed().as_secs_f64();
                
                // Parse the AI response into structured format
                let structured_result = self.parse_ai_response(&ai_response, &request.data);
                
                // Update the analysis result
                analysis_result.analysis_result = structured_result.clone();
                analysis_result.status = AnalysisStatus::Completed;
                analysis_result.processing_time = processing_time;
                analysis_result.insights_count = self.count_insights(&structured_result);
                analysis_result.recommendations_count = self.count_recommendations(&structured_result);

                // Update in storage
                {
                    let mut results = self.analysis_results.write().await;
                    if let Some(integration_results) = results.get_mut(&integration.id) {
                        if let Some(last_result) = integration_results.last_mut() {
                            *last_result = analysis_result.clone();
                        }
                    }
                }

                // Send webhook notification if configured
                if let Some(webhook_url) = &integration.webhook_url {
                    self.send_webhook_notification(webhook_url, &analysis_result).await;
                }

                // Send callback notification if provided
                if let Some(callback_url) = &request.callback_url {
                    self.send_callback_notification(callback_url, &analysis_result).await;
                }

                Ok(analysis_result)
            }
            Err(e) => {
                // Update result with error
                analysis_result.status = AnalysisStatus::Failed;
                analysis_result.analysis_result = serde_json::json!({
                    "error": format!("Analysis failed: {}", e)
                });

                {
                    let mut results = self.analysis_results.write().await;
                    if let Some(integration_results) = results.get_mut(&integration.id) {
                        if let Some(last_result) = integration_results.last_mut() {
                            *last_result = analysis_result.clone();
                        }
                    }
                }

                Err(format!("Analysis failed: {}", e))
            }
        }
    }

    /// Get analysis results for an integration
    pub async fn get_analysis_results(&self, integration_id: &str, limit: Option<usize>) -> Vec<IntegrationAnalysisResult> {
        let results = self.analysis_results.read().await;
        if let Some(integration_results) = results.get(integration_id) {
            let mut sorted_results = integration_results.clone();
            sorted_results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            
            if let Some(limit) = limit {
                sorted_results.truncate(limit);
            }
            
            sorted_results
        } else {
            Vec::new()
        }
    }

    /// Get dashboard statistics
    pub async fn get_dashboard_stats(&self) -> serde_json::Value {
        let integrations = self.integrations.read().await;
        let results = self.analysis_results.read().await;

        let total_integrations = integrations.len();
        let active_integrations = integrations.values()
            .filter(|i| matches!(i.status, IntegrationStatus::Active))
            .count();

        let total_analyses: usize = results.values().map(|v| v.len()).sum();
        let successful_analyses: usize = results.values()
            .flat_map(|v| v.iter())
            .filter(|r| matches!(r.status, AnalysisStatus::Completed))
            .count();

        let recent_analyses: usize = results.values()
            .flat_map(|v| v.iter())
            .filter(|r| r.created_at > Utc::now() - chrono::Duration::hours(24))
            .count();

        serde_json::json!({
            "total_integrations": total_integrations,
            "active_integrations": active_integrations,
            "total_analyses": total_analyses,
            "successful_analyses": successful_analyses,
            "recent_analyses_24h": recent_analyses,
            "success_rate": if total_analyses > 0 { successful_analyses as f64 / total_analyses as f64 } else { 0.0 }
        })
    }

    /// Parse AI response into structured format
    fn parse_ai_response(&self, ai_response: &str, original_data: &serde_json::Value) -> serde_json::Value {
        // Try to parse as JSON first
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(ai_response) {
            return json;
        }

        // If not JSON, create structured format
        serde_json::json!({
            "summary": ai_response,
            "insights": self.extract_insights(ai_response),
            "recommendations": self.extract_recommendations(ai_response),
            "metrics": {
                "data_points": self.count_data_points(original_data),
                "analysis_confidence": 0.85,
                "processing_timestamp": Utc::now().to_rfc3339()
            },
            "original_data_sample": self.sample_data(original_data)
        })
    }

    /// Extract insights from AI response
    fn extract_insights(&self, response: &str) -> Vec<serde_json::Value> {
        let mut insights = Vec::new();
        
        // Simple pattern matching for insights
        if response.contains("pattern") || response.contains("trend") {
            insights.push(serde_json::json!({
                "type": "pattern",
                "title": "Pattern Detected",
                "description": "Data patterns identified in the analysis",
                "confidence": 0.85
            }));
        }

        if response.contains("anomaly") || response.contains("outlier") {
            insights.push(serde_json::json!({
                "type": "anomaly",
                "title": "Anomaly Found",
                "description": "Unusual data points detected",
                "confidence": 0.75
            }));
        }

        insights
    }

    /// Extract recommendations from AI response
    fn extract_recommendations(&self, response: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if response.contains("optimize") {
            recommendations.push("Consider optimizing data processing".to_string());
        }
        
        if response.contains("monitor") {
            recommendations.push("Implement continuous monitoring".to_string());
        }
        
        if response.is_empty() {
            recommendations.push("Review analysis results for actionable insights".to_string());
        }

        recommendations
    }

    /// Count insights in structured result
    fn count_insights(&self, result: &serde_json::Value) -> usize {
        if let Some(insights) = result.get("insights").and_then(|v| v.as_array()) {
            insights.len()
        } else {
            0
        }
    }

    /// Count recommendations in structured result
    fn count_recommendations(&self, result: &serde_json::Value) -> usize {
        if let Some(recommendations) = result.get("recommendations").and_then(|v| v.as_array()) {
            recommendations.len()
        } else {
            0
        }
    }

    /// Count data points in original data
    fn count_data_points(&self, data: &serde_json::Value) -> usize {
        match data {
            serde_json::Value::Array(arr) => arr.len(),
            serde_json::Value::Object(obj) => obj.len(),
            _ => 1,
        }
    }

    /// Sample data for display
    fn sample_data(&self, data: &serde_json::Value) -> serde_json::Value {
        match data {
            serde_json::Value::Array(arr) => {
                if arr.len() > 3 {
                    serde_json::json!({
                        "type": "array",
                        "length": arr.len(),
                        "sample": &arr[0..3]
                    })
                } else {
                    data.clone()
                }
            }
            serde_json::Value::Object(obj) => {
                if obj.len() > 5 {
                    let mut sample = serde_json::Map::new();
                    for (i, (key, value)) in obj.iter().enumerate() {
                        if i >= 5 { break; }
                        sample.insert(key.clone(), value.clone());
                    }
                    serde_json::json!({
                        "type": "object",
                        "total_keys": obj.len(),
                        "sample": sample
                    })
                } else {
                    data.clone()
                }
            }
            _ => data.clone(),
        }
    }

    /// Send webhook notification
    async fn send_webhook_notification(&self, webhook_url: &str, result: &IntegrationAnalysisResult) {
        // Implementation would send HTTP POST to webhook_url
        log::info!("Sending webhook notification to: {}", webhook_url);
        // TODO: Implement actual webhook sending
    }

    /// Send callback notification
    async fn send_callback_notification(&self, callback_url: &str, result: &IntegrationAnalysisResult) {
        // Implementation would send HTTP POST to callback_url
        log::info!("Sending callback notification to: {}", callback_url);
        // TODO: Implement actual callback sending
    }
}

/// Create integration routes
pub fn create_integration_routes() -> Router<Arc<IntegrationManager>> {
    Router::new()
        .route("/integrations", post(create_integration))
        .route("/integrations", get(list_integrations))
        .route("/integrations/:id", get(get_integration))
        .route("/integrations/:id", delete(delete_integration))
        .route("/integrations/:id/results", get(get_integration_results))
        .route("/integrations/:id/results/:result_id", get(get_analysis_result))
        .route("/integrations/stats", get(get_dashboard_stats))
        .route("/analyze", post(process_analysis))
}

// Handler functions
async fn create_integration(
    State(manager): State<Arc<IntegrationManager>>,
    Json(request): Json<CreateIntegrationRequest>,
) -> Result<Json<Integration>, StatusCode> {
    match manager.create_integration(request).await {
        Ok(integration) => Ok(Json(integration)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_integrations(
    State(manager): State<Arc<IntegrationManager>>,
) -> Json<Vec<Integration>> {
    Json(manager.list_integrations().await)
}

async fn get_integration(
    State(manager): State<Arc<IntegrationManager>>,
    Path(id): Path<String>,
) -> Result<Json<Integration>, StatusCode> {
    match manager.get_integration(&id).await {
        Some(integration) => Ok(Json(integration)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_integration(
    State(manager): State<Arc<IntegrationManager>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    if manager.delete_integration(&id).await {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn get_integration_results(
    State(manager): State<Arc<IntegrationManager>>,
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<IntegrationAnalysisResult>>, StatusCode> {
    let limit = params.get("limit").and_then(|l| l.parse().ok());
    Ok(Json(manager.get_analysis_results(&id, limit).await))
}

async fn get_analysis_result(
    State(manager): State<Arc<IntegrationManager>>,
    Path((integration_id, result_id)): Path<(String, String)>,
) -> Result<Json<IntegrationAnalysisResult>, StatusCode> {
    let results = manager.get_analysis_results(&integration_id, None).await;
    
    if let Some(result) = results.into_iter().find(|r| r.id == result_id) {
        Ok(Json(result))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn get_dashboard_stats(
    State(manager): State<Arc<IntegrationManager>>,
) -> Json<serde_json::Value> {
    Json(manager.get_dashboard_stats().await)
}

async fn process_analysis(
    State(manager): State<Arc<IntegrationManager>>,
    Json(request): Json<AnalysisRequest>,
) -> Result<Json<IntegrationAnalysisResult>, StatusCode> {
    // This would need access to the Ollama client
    // For now, return a mock response
    Err(StatusCode::NOT_IMPLEMENTED)
}
