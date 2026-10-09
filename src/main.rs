/// Kite x402 service template (Rust + Axum).
///
/// Wraps an existing HTTP API behind x402 payments settled on the Kite chain.
/// Requests to `/v1/*` return HTTP 402 until the caller attaches a valid
/// `PAYMENT-SIGNATURE`; the payment is verified by the facilitator, the request
/// is proxied to `UPSTREAM_URL`, and the payment is settled only if the upstream
/// answered with a non-error status.
///
/// ## Environment variables
///
/// See `.env.example` for the full list.

use std::sync::Arc;

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use x402_axum::X402Middleware;

mod kite;
mod env;
mod proxy;

use env::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load .env file (optional).
    let _ = dotenvy::dotenv();

    // --- Validate required env vars ---
    let pay_to = env("PAY_TO", "");
    if pay_to.is_empty() {
        panic!("PAY_TO is required: the Kite wallet address that receives payments");
    }

    let chain = kite::kite_chain_by_name(&env("KITE_NETWORK", "mainnet"));
    let price_raw = env("PRICE_USD", "0.001");
    let price = if price_raw.starts_with('$') {
        price_raw.clone()
    } else {
        format!("${price_raw}")
    };

    let upstream_url = env("UPSTREAM_URL", "");
    if upstream_url.is_empty() {
        panic!("UPSTREAM_URL is required, e.g. https://api.example.com");
    }

    let service_desc = env("SERVICE_DESCRIPTION", "Paid API wrapped for the Kite network");

    let port: u16 = env("PORT", "8080").parse().expect("PORT must be a number");

    // --- x402 setup ---
    let facilitator_url = env("FACILITATOR_URL", &chain.default_facilitator_url);
    let x402 = X402Middleware::new(&facilitator_url);

    // Kite price tag — uses V1 exact payment scheme.
    let price_tag = chain.price_tag(&pay_to, &price.trim_start_matches('$'));

    tracing::info!(
        "kite x402 service on :{} -> {} (network {}, {} per call to {})",
        port,
        upstream_url,
        chain.network,
        price,
        pay_to
    );

    // --- Shared proxy state ---
    let app_state = Arc::new(proxy::AppState::from_env()?);

    // --- Build router ---
    let app = Router::new()
        // Health check — always free (no x402 layer).
        .route("/healthz", get(health_handler))
        // Paid routes under /v1/ — protected by x402 middleware.
        .route(
            "/v1/*path",
            any(proxy::proxy_handler).layer(
                x402
                    .with_price_tag(price_tag)
                    .with_description(service_desc)
                    .with_mime_type("application/json".into()),
            ),
        )
        // Shared state for the proxy handler.
        .with_state(app_state)
        // Middleware stack.
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // --- Start server ---
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("bind failed");
    axum::serve(listener, app)
        .await
        .expect("server failed");

    Ok(())
}

/// GET /healthz — free endpoint for health checks (no payment required).
async fn health_handler() -> impl IntoResponse {
    let chain = kite::kite_chain_by_name(&env("KITE_NETWORK", "mainnet"));
    let price_raw = env("PRICE_USD", "0.001");
    let price = if price_raw.starts_with('$') {
        price_raw
    } else {
        format!("${price_raw}")
    };
    (
        StatusCode::OK,
        serde_json::json!({
            "ok": true,
            "network": chain.network,
            "asset": chain.asset_symbol,
            "price": price,
        })
        .to_string(),
    )
}
