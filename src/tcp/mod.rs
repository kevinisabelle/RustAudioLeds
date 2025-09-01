mod helpers;
mod server;

use crate::settings::{AnimationMode, DisplayMode, Settings};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{
    net::SocketAddr,
    sync::Arc
    ,
};
use crate::color::Color;

#[derive(Clone)]
pub struct HttpServerState {
    pub bind_addr: SocketAddr,
    pub bearer_token: String,
    pub settings: Arc<Mutex<Settings>>,
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
            settings: state,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Command
{
    pub smooth_size: Option<usize>,
    pub gain: Option<f32>,
    pub fps: Option<usize>,
    pub color1: Option<Color>,
    pub color2: Option<Color>,
    pub color3: Option<Color>,
    pub fft_size: Option<usize>,
    pub frequencies:  Option<Vec<f32>>,
    pub gains: Option<Vec<f32>>,
    pub skew: Option<f32>,
    pub brightness: Option<f32>,
    pub display_mode: Option<DisplayMode>,
    pub animation_mode: Option<AnimationMode>,
}

async fn info(State(state): State<HttpServerState>) -> impl IntoResponse {
    println!("GET /api/v1/info");
    Json(state.settings.lock().unwrap().clone())
}

async fn post_command(
    State(state): State<HttpServerState>,
    headers: HeaderMap,
    Json(cmd): Json<Command>,
) -> Result<impl IntoResponse, StatusCode> {
    // check_auth(&headers, shared.bearer_token)?;

    println!("POST /api/v1/command");
    if let Some(b) = cmd.brightness {
        state.settings.lock().unwrap().brightness = b.min(1.0).max(0.0);
    }

    if let Some(s) = cmd.smooth_size {
        state.settings.lock().unwrap().smooth_size = s;
    }

    if let Some(g) = cmd.gain {
        state.settings.lock().unwrap().gain = g;
    }

    if let Some(f) = cmd.fps {
        state.settings.lock().unwrap().fps = f;
    }

    if let Some(c1) = cmd.color1 {
        state.settings.lock().unwrap().color1 = c1;
    }

    if let Some(c2) = cmd.color2 {
        state.settings.lock().unwrap().color2 = c2;
    }

    if let Some(c3) = cmd.color3 {
        state.settings.lock().unwrap().color3 = c3;
    }

    if let Some(s) = cmd.fft_size {
        state.settings.lock().unwrap().set_fft_size(s);
    }

    if let Some(freqs) = cmd.frequencies {
        state.settings.lock().unwrap().frequencies = freqs;
    }

    if let Some(gains) = cmd.gains {
        state.settings.lock().unwrap().gains = gains;
    }

    if let Some(s) = cmd.skew {
        state.settings.lock().unwrap().skew = s;
    }

    if let Some(d) = cmd.display_mode {
        state.settings.lock().unwrap().display_mode = d;
    }

    if let Some(a) = cmd.animation_mode {
        state.settings.lock().unwrap().animation_mode = a;
    }

    Ok((StatusCode::OK, Json(state.settings.lock().unwrap().clone())))
}

pub async fn start_https_server(
    state: HttpServerState
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("Starting HTTPS server...");

    let app = Router::new()
        .route("/api/v1/info", get(info))
        .route("/api/v1/command", post(post_command))
        .with_state(state.clone());

    println!("HTTPS listening on https://{} ...", state.clone().bind_addr.clone());
    let config = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem",
    ).await.unwrap();

    let addr = state.bind_addr;
    println!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}