use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::{HeaderMap, StatusCode};
pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn check_auth(headers: &HeaderMap, token: Option<&str>) -> Result<(), StatusCode> {
    if let Some(expected) = token {
        if let Some(value) = headers.get("authorization") {
            if let Ok(s) = value.to_str() {
                if let Some(given) = s.strip_prefix("Bearer ") {
                    if subtle_equals(given.as_bytes(), expected.as_bytes()) {
                        return Ok(());
                    }
                }
            }
        }
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(())
}

// Constant-time-ish equality to avoid timing leaks on tokens.
fn subtle_equals(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}