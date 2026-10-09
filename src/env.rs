/// Environment variable helper — mirrors `env(key, fallback)` from the Go/TS templates.
use std::env;

/// Read an environment variable, trimming whitespace.
/// Returns `fallback` (default `""`) when unset or empty.
pub fn env(key: &str, fallback: &str) -> String {
    env::var(key).map(|v| v.trim().to_string()).unwrap_or_else(|_| fallback.to_string())
}
