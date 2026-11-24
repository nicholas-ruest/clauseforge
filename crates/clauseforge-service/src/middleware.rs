use axum::body::Body;
use axum::extract::Request;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Principal(pub String);

#[derive(Clone, Debug)]
pub struct CorrelationId(pub String);

pub async fn correlation(mut request: Request, next: Next) -> Response {
    let id = request
        .headers()
        .get("x-correlation-id")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    request.extensions_mut().insert(CorrelationId(id.clone()));
    let mut response = next.run(request).await;
    if let Ok(value) = HeaderValue::from_str(&id) {
        response.headers_mut().insert("x-correlation-id", value);
    }
    response
}

fn request_correlation(request: &Request) -> String {
    request
        .extensions()
        .get::<CorrelationId>()
        .map(|value| value.0.clone())
        .unwrap_or_else(|| "correlation-unavailable".into())
}

pub async fn authn(mut request: Request, next: Next) -> Response {
    if request.method() == Method::GET {
        request
            .extensions_mut()
            .insert(Principal("reader@example.com".into()));
        return next.run(request).await;
    }
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let expected =
        std::env::var("CLAUSEFORGE_TOKEN").unwrap_or_else(|_| "clauseforge-reviewer".into());
    if token != format!("Bearer {expected}") {
        return error_response(
            StatusCode::UNAUTHORIZED,
            "authentication required",
            &request_correlation(&request),
        );
    }
    let reviewer = request
        .headers()
        .get("x-reviewer")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("sre@example.com")
        .to_string();
    request.extensions_mut().insert(Principal(reviewer));
    next.run(request).await
}

pub async fn authz_policy(request: Request, next: Next) -> Response {
    if request.method() != Method::GET {
        let role = request
            .headers()
            .get("x-role")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        if role != "amendment-reviewer" {
            return error_response(
                StatusCode::FORBIDDEN,
                "policy denied: amendment-reviewer role required",
                &request_correlation(&request),
            );
        }
    }
    next.run(request).await
}

pub async fn validate_request(request: Request, next: Next) -> Response {
    if request.method() == Method::POST && request.uri().path() == "/api/v1/amendments" {
        let correlation = request_correlation(&request);
        let content_type = request
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        if !content_type.starts_with("application/json") {
            return error_response(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "application/json content type required",
                &correlation,
            );
        }
        if request
            .headers()
            .get(header::CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .is_some_and(|length| length > 65_536)
        {
            return error_response(
                StatusCode::PAYLOAD_TOO_LARGE,
                "request body exceeds 65536 bytes",
                &correlation,
            );
        }
    }
    next.run(request).await
}

fn known_api_path(path: &str) -> bool {
    matches!(path, "/api/v1/amendments" | "/api/v1/audit")
        || path
            .strip_prefix("/api/v1/amendments/")
            .is_some_and(|correlation| !correlation.is_empty() && !correlation.contains('/'))
}

pub async fn normalize_errors(request: Request, next: Next) -> Response {
    let correlation = request_correlation(&request);
    let path = request.uri().path().to_owned();
    if path.starts_with("/api/") && !known_api_path(&path) {
        return error_response(StatusCode::NOT_FOUND, "API route not found", &correlation);
    }
    let response = next.run(request).await;
    if !(response.status().is_client_error() || response.status().is_server_error()) {
        return response;
    }
    let is_json = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("application/json"));
    if is_json {
        response
    } else {
        let status = response.status();
        error_response(
            status,
            status.canonical_reason().unwrap_or("request failed"),
            &correlation,
        )
    }
}

pub fn error_response(status: StatusCode, message: &str, correlation: &str) -> Response<Body> {
    let body = serde_json::json!({"code":status.as_u16().to_string(),"message":message,"correlation_id":correlation});
    (status, axum::Json(body)).into_response()
}

pub async fn timeouts(request: Request, next: Next) -> Response {
    let correlation = request_correlation(&request);
    match tokio::time::timeout(std::time::Duration::from_secs(120), next.run(request)).await {
        Ok(response) => response,
        Err(_) => error_response(
            StatusCode::GATEWAY_TIMEOUT,
            "request deadline exceeded",
            &correlation,
        ),
    }
}
