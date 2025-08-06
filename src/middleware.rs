use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use tracing::{debug, error, warn};

type HmacSha256 = Hmac<Sha256>;

pub async fn verify_github_signature(
    State(secret): State<String>,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let signature_header = headers.get("x-hub-signature-256");
    
    if signature_header.is_none() {
        warn!("Missing X-Hub-Signature-256 header");
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let signature = signature_header
        .unwrap()
        .to_str()
        .map_err(|_| {
            error!("Invalid signature header encoding");
            StatusCode::BAD_REQUEST
        })?;
    
    if !signature.starts_with("sha256=") {
        error!("Invalid signature format, expected 'sha256=' prefix");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let expected_signature = &signature[7..]; // Remove "sha256=" prefix
    
    // Extract body for signature verification
    let (parts, body) = request.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| {
            error!("Failed to read request body");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Verify signature
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| {
            error!("Invalid HMAC key");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    mac.update(&body_bytes);
    let computed_signature = hex::encode(mac.finalize().into_bytes());
    
    if computed_signature != expected_signature {
        warn!("Signature verification failed");
        debug!("Expected: {}, Got: {}", expected_signature, computed_signature);
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    debug!("GitHub webhook signature verified successfully");
    
    // Reconstruct request with the body we consumed
    let request = Request::from_parts(parts, Body::from(body_bytes));
    
    Ok(next.run(request).await)
}