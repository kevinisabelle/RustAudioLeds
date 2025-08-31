mod helpers;
mod server;

use crate::settings::Settings;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use sha2::Digest;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::tcp::helpers::{check_auth};

#[derive(Clone)]
pub struct HttpServerState {
    pub bind_addr: SocketAddr,
    pub bearer_token: String,
    pub state: Arc<Mutex<Settings>>,
}

impl HttpServerState {
    pub fn new(
        bind_addr: SocketAddr,
        bearer_token: String,
        state: Arc<Mutex<Settings>>,
    ) -> Self {
        HttpServerState {
            bind_addr,
            bearer_token,
            state,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Command
{
    pub power: Option<bool>,
    pub brightness: Option<f32>,
}

async fn info(State(state): State<HttpServerState>) -> impl IntoResponse {
    Json(state.state.lock().unwrap().clone())
}

async fn post_command(
    State(shared): State<HttpServerState>,
    headers: HeaderMap,
    Json(cmd): Json<Command>,
) -> Result<impl IntoResponse, StatusCode> {
    // check_auth(&headers, shared.bearer_token)?;

    let mut st = shared.state.lock().unwrap().clone();
    if let Some(b) = cmd.brightness {
        st.brightness = b.min(1.0).max(0.0);
    }

    Ok((StatusCode::OK, Json(st.clone())))
}

pub async fn start_https_server(
    state: HttpServerState
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let app = Router::new()
        .route("/api/v1/info", get(info))
        .route("/api/v1/command", post(post_command))
        .with_state(state.clone());

    println!("HTTPS listening on https://{} ...", state.clone().bind_addr.clone());
    let config = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem",
    )
        .await
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}