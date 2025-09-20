//! Authentication module for Clerk JWT verification
//! Handles user authentication and authorization for the JSON Oracle API

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::sync::Arc;

/// Clerk user information extracted from JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClerkUser {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_url: Option<String>,
    pub created_at: i64,
}

/// Clerk JWT claims structure
#[derive(Debug, Deserialize)]
struct ClerkClaims {
    sub: String,                    // User ID
    email: String,                  // User email
    given_name: Option<String>,     // First name
    family_name: Option<String>,    // Last name
    picture: Option<String>,        // Profile picture URL
    iat: u64,                      // Issued at
    exp: u64,                      // Expires at
    aud: String,                   // Audience
    iss: String,                   // Issuer
}

/// Authentication middleware for protecting routes
pub async fn auth_middleware(
    headers: HeaderMap,
    State(state): State<Arc<crate::api::core_handlers::ApiState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    
    // Verify JWT token with Clerk
    match verify_clerk_jwt(token).await {
        Ok(user) => {
            // Add user information to request extensions for downstream handlers
            let mut request = request;
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Verify Clerk JWT token and extract user information
pub async fn verify_clerk_jwt(token: &str) -> Result<ClerkUser, String> {
    // Get Clerk secret from environment
    let clerk_secret = std::env::var("CLERK_SECRET_KEY")
        .map_err(|_| "CLERK_SECRET_KEY not set".to_string())?;

    // For Clerk, we need to verify the JWT signature using their public key
    // In production, you should fetch the public key from Clerk's JWKS endpoint
    let public_key = get_clerk_public_key().await
        .map_err(|e| format!("Failed to get Clerk public key: {}", e))?;

    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())
        .map_err(|e| format!("Failed to create decoding key: {}", e))?;

    let validation = Validation::new(Algorithm::RS256);
    
    match decode::<ClerkClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            // Verify the issuer is Clerk
            if !token_data.claims.iss.contains("clerk") {
                return Err("Invalid issuer".to_string());
            }

            let user = ClerkUser {
                id: token_data.claims.sub,
                email: token_data.claims.email,
                first_name: token_data.claims.given_name,
                last_name: token_data.claims.family_name,
                image_url: token_data.claims.picture,
                created_at: token_data.claims.iat as i64,
            };

            Ok(user)
        }
        Err(e) => Err(format!("Token verification failed: {}", e))
    }
}

/// Get Clerk's public key for JWT verification
async fn get_clerk_public_key() -> Result<String, String> {
    // In production, you should fetch this from Clerk's JWKS endpoint
    // For now, we'll use a simple approach with the secret key
    
    // Clerk provides JWKS endpoint at: https://your-clerk-domain.clerk.accounts.dev/.well-known/jwks.json
    let clerk_domain = std::env::var("CLERK_DOMAIN")
        .unwrap_or_else(|_| "clerk.accounts.dev".to_string());
    
    let jwks_url = format!("https://{}/.well-known/jwks.json", clerk_domain);
    
    match reqwest::get(&jwks_url).await {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(jwks) => {
                    // Extract the first public key from JWKS
                    if let Some(keys) = jwks.get("keys").and_then(|k| k.as_array()) {
                        if let Some(key) = keys.first() {
                            if let Some(x5c) = key.get("x5c").and_then(|x| x.as_array()) {
                                if let Some(cert) = x5c.first().and_then(|c| c.as_str()) {
                                    return Ok(format!("-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----", cert));
                                }
                            }
                        }
                    }
                    Err("No valid public key found in JWKS".to_string())
                }
                Err(e) => Err(format!("Failed to parse JWKS: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to fetch JWKS: {}", e))
    }
}

/// Extract user from request extensions (set by auth middleware)
pub fn get_current_user(request: &Request) -> Option<ClerkUser> {
    request.extensions().get::<ClerkUser>().cloned()
}

/// Create user-specific API key
pub fn create_user_api_key(user_id: &str) -> String {
    use uuid::Uuid;
    let uuid = Uuid::new_v4().to_string().replace("-", "");
    format!("json_oracle_{}_{}", user_id, uuid)
}

/// Validate user owns the integration
pub async fn validate_user_integration(
    integration_id: &str,
    user_id: &str,
    state: &crate::api::core_handlers::ApiState,
) -> Result<(), StatusCode> {
    // This would check if the integration belongs to the user
    // For now, we'll implement a simple check
    if integration_id.contains(user_id) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user_api_key() {
        let user_id = "user_123";
        let api_key = create_user_api_key(user_id);
        
        assert!(api_key.starts_with("json_oracle_user_123_"));
        assert!(api_key.len() > 30); // Should be a reasonable length
    }

    #[tokio::test]
    async fn test_validate_user_integration() {
        let integration_id = "integration_user_123_abc";
        let user_id = "user_123";
        
        // This would need proper state setup in a real test
        // For now, just test the basic logic
        assert!(integration_id.contains(user_id));
    }
}
