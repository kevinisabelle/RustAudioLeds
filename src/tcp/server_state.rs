use crate::color::Color;
use crate::settings::{AnimationMode, DisplayMode, Settings};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{
    net::SocketAddr,
    sync::Arc
    ,
};

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
