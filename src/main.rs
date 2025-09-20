//! Main entry point for the AI JSON Analysis API
//! Compatible with both traditional servers and serverless platforms

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Get port from environment or default to 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    log::info!("🚀 Starting AI JSON Analysis API on port {}", port);
    
    // Start the API server
    ai_json_analysis_api::api::start_api_server(port).await?;
    
    Ok(())
}
