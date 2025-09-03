use crate::presets;
use crate::presets::{string_to_name_bytes, Preset, PresetInfo};
use crate::tcp::server_state::{Command, HttpServerState};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_server::tls_rustls::RustlsConfig;

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

async fn list_presets_handler() -> impl IntoResponse {
    println!("GET /api/v1/presets");
    match presets::list_presets() {
        Ok(presets) => Ok((StatusCode::OK, Json(presets))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_preset_handler(Path(id): Path<u8>) -> impl IntoResponse {
    println!("GET /api/v1/presets/{}", id);
    match presets::load_preset(id) {
        Ok(preset) => Ok((StatusCode::OK, Json(preset))),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn create_preset_handler(Json(preset): Json<Preset>) -> impl IntoResponse {
    println!("POST /api/v1/presets");
    match presets::save_preset(&preset) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn save_current_settings_as_preset(State(state): State<HttpServerState>, Json(presetInfo) : Json<PresetInfo>) -> impl IntoResponse {
    println!("POST /api/v1/presets/save_current");
    let preset = Preset::from_settings(&state.settings.lock().unwrap(), presetInfo.index, string_to_name_bytes(presetInfo.name.as_str()));
    
    match presets::save_preset(&preset) {
        Ok(_) => {
            state.settings.lock().unwrap().active_preset = presetInfo.index as usize;
            Ok(StatusCode::CREATED)
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn delete_preset_handler(Path(id): Path<u8>) -> impl IntoResponse {
    println!("DELETE /api/v1/presets/{}", id);
    match presets::delete_preset(id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn activate_preset_handler(
    State(state): State<HttpServerState>,
    Path(id): Path<u8>,
) -> impl IntoResponse {
    println!("POST /api/v1/presets/{}/activate", id);
    match presets::load_preset(id) {
        Ok(preset) => {
            let mut settings = state.settings.lock().unwrap();
            preset.apply_to_settings(&mut settings);
            Ok(StatusCode::OK)
        }
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

pub async fn start_https_server(
    state: HttpServerState
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("Starting HTTPS server...");

    let app = Router::new()
        .route("/api/v1/info", get(info))
        .route("/api/v1/command", post(post_command))
        .route("/api/v1/presets", get(list_presets_handler).post(create_preset_handler))
        .route("/api/v1/presets/{id}", get(get_preset_handler).delete(delete_preset_handler))
        .route("/api/v1/presets/{id}/activate", post(activate_preset_handler))
        .route("/api/v1/presets/save_current", post(save_current_settings_as_preset))
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