use axum::body::Body;
use axum::extract::{Request, State};
use axum::response::Response;
use http::{HeaderName, StatusCode, Uri};
/// Reverse proxy — forwards paid requests to the upstream API.
///
/// Mirrors the upstream proxy logic in the Go/Gin and TypeScript/Express templates:
/// - Strips the `/v1` prefix so `/v1/forecast` becomes `/forecast` at the upstream.
/// - Removes hop-by-hop headers and `PAYMENT-SIGNATURE`.
/// - Injects the upstream credential (if configured).
/// - Returns 502 if the upstream is unreachable (prevents settlement).
use std::collections::HashSet;
use std::sync::Arc;

use crate::env::env;

/// Hop-by-hop headers that must not be forwarded.
fn hop_by_hop() -> HashSet<&'static str> {
    ["connection", "keep-alive", "transfer-encoding", "te", "trailer", "upgrade", "host", "content-length"]
        .into_iter()
        .collect()
}

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub upstream_base: Arc<str>,
    pub upstream_auth_header: Arc<str>,
    pub upstream_auth_value: Arc<str>,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn from_env() -> Result<Self, &'static str> {
        let upstream = env("UPSTREAM_URL", "");
        if upstream.is_empty() {
            return Err("UPSTREAM_URL is required, e.g. https://api.example.com");
        }
        Ok(Self {
            upstream_base: upstream.into(),
            upstream_auth_header: env("UPSTREAM_AUTH_HEADER", "Authorization").into(),
            upstream_auth_value: env("UPSTREAM_AUTH_VALUE", "").into(),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
        })
    }
}

/// Proxy handler for `/v1/*path` — forwards to the upstream API.
///
/// Return 502 on upstream failure (prevents x402 settlement).
pub async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    // Build the upstream URL by stripping `/v1` prefix.
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| {
            let path = pq.path().strip_prefix("/v1").unwrap_or(pq.path());
            if let Some(query) = pq.query() { format!("{}?{}", path, query) } else { path.to_string() }
        })
        .unwrap_or_else(|| "/".to_string());

    let upstream_uri: Uri = format!("{}{}", state.upstream_base.as_ref(), path_and_query).parse().map_err(|e| {
        tracing::error!("Invalid upstream URI: {e}");
        (StatusCode::BAD_GATEWAY, format!("Invalid upstream URI: {e}"))
    })?;

    // Build the upstream request.
    let method = req.method().clone();
    let mut up_req = state.http_client.request(method, upstream_uri.to_string());

    // Forward headers, skipping hop-by-hop and payment-signature.
    let hop = hop_by_hop();
    for (name, value) in req.headers() {
        let name_str = name.as_str().to_lowercase();
        if hop.contains(name_str.as_str()) || name_str == "payment-signature" {
            continue;
        }
        up_req = up_req.header(name, value);
    }

    // Inject upstream credential.
    let auth_val = state.upstream_auth_value.as_ref();
    if !auth_val.is_empty() {
        let header_name: HeaderName =
            state.upstream_auth_header.as_ref().parse().unwrap_or(HeaderName::from_static("authorization"));
        up_req = up_req.header(header_name, auth_val);
    }

    // Forward body for methods that carry one.
    let method_str = req.method().as_str();
    let has_body = !["GET", "HEAD"].contains(&method_str);
    if has_body {
        let body_bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024 * 10).await.map_err(|e| {
            tracing::error!("Failed to read body: {e}");
            (StatusCode::BAD_REQUEST, format!("Body read error: {e}"))
        })?;
        up_req = up_req.body(body_bytes);
    }

    // Execute upstream request.
    let up_res = state
        .http_client
        .execute(up_req.build().map_err(|e| {
            tracing::error!("Failed to build request: {e}");
            (StatusCode::BAD_GATEWAY, format!("Request build error: {e}"))
        })?)
        .await
        .map_err(|e| {
            tracing::error!("Upstream unreachable: {e}");
            // 502 is >= 400, so the x402 middleware will NOT settle the charge.
            (StatusCode::BAD_GATEWAY, format!("Upstream unreachable: {e}"))
        })?;

    // Build downstream response.
    let status = up_res.status();
    let mut response = Response::builder().status(status);

    let hop = hop_by_hop();
    let headers = response.headers_mut().unwrap();
    for (name, value) in up_res.headers() {
        let name_str = name.as_str().to_lowercase();
        if !hop.contains(name_str.as_str()) && name_str != "content-encoding" {
            headers.insert(name, value.clone());
        }
    }

    let body_bytes = up_res.bytes().await.unwrap_or_default();
    response.body(Body::from(body_bytes)).map_err(|e| {
        tracing::error!("Response build error: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Response error: {e}"))
    })
}

#[cfg(test)]
mod tests {
    // Integration tests live in tests/ directory.
}
