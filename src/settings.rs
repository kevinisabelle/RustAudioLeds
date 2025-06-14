﻿use crate::color::{color_from_string, Color};
use crate::DEFAULT_SMOOTH_SIZE;
use crate::constants::{DEFAULT_SKEW, FFT_SIZE, FPS, GAIN, SAMPLE_RATE};

#[derive(Debug, PartialEq, Clone)]
pub enum DisplayMode {
    Spectrum = 0,
    Oscilloscope = 1,
    ColorGradient = 2,
}

impl DisplayMode {
    pub fn from_u8(value: u8) -> Option<DisplayMode> {
        match value {
            0 => Some(DisplayMode::Spectrum),
            1 => Some(DisplayMode::Oscilloscope),
            2 => Some(DisplayMode::ColorGradient),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AnimationMode {
    Full = 0,
    FullWithMax = 1,
    Points = 2,
    FullMiddle = 3,
    FullMiddleWithMax = 4,
    PointsMiddle = 5,
}

impl AnimationMode {
    pub fn from_u8(value: u8) -> Option<AnimationMode> {
        match value {
            0 => Some(AnimationMode::Full),
            1 => Some(AnimationMode::FullWithMax),
            2 => Some(AnimationMode::Points),
            3 => Some(AnimationMode::FullMiddle),
            4 => Some(AnimationMode::FullMiddleWithMax),
            5 => Some(AnimationMode::PointsMiddle),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings  {
    pub smooth_size: usize,
    pub gain: f32,
    pub fps: usize,
    pub color1: Color,
    pub color2: Color,
    pub color3: Color,
    pub fft_size: usize,
    pub frequencies:  Vec<f32>,
    pub gains: Vec<f32>,
    pub skew: f32,
    pub brightness: f32,
    pub display_mode: DisplayMode,
    pub animation_mode: AnimationMode,
    pub led_buffer: Vec<u8>,
    pub cached_df: f32,
    pub selected_preset: usize,
    pub active_preset: usize,
}

impl Settings
{
    pub fn set_fft_size(&mut self, fft_size: usize) {
        self.fft_size = fft_size;
        self.cached_df = SAMPLE_RATE as f32 / self.fft_size as f32
    }
}

pub fn get_config() -> Settings {

    let mut settings = Settings {
        smooth_size: DEFAULT_SMOOTH_SIZE,
        gain: GAIN,
        fps: FPS,
        color1: color_from_string("blue"),
        color2: color_from_string("red"),
        color3: color_from_string("magenta"),
        fft_size: FFT_SIZE,
        skew: DEFAULT_SKEW,
        brightness: 1.0,
        display_mode: DisplayMode::Spectrum,
        frequencies: vec![41.0, 55.0, 65.0, 82.0, 110.0, 146.0, 220.0, 261.0, 329.0, 392.0,
                          440.0, 523.0, 880.0, 987.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7500.0,
                          9000.0, 13000.0],
        gains: vec![1.3, 1.2, 1.1, 1.0, 1.0, 1.0, 1.0, 0.85, 0.75, 0.75,
                    0.75, 0.75, 0.75, 0.75, 1.0, 1.0, 1.0, 1.0, 1.2, 3.0,
                    4.0, 4.0],
        animation_mode: AnimationMode::Full,
        led_buffer: vec![0; 3 * 22 * 12 + 1], // Assuming 22 LEDs, 3 bytes per LED + 1 end marker,
        cached_df: 0.0,
        selected_preset: 0,
        active_preset: 255,
    };

    settings.set_fft_size(FFT_SIZE);

    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--smooth" | "-s" => {
                if let Some(val) = args.next() {
                    settings.smooth_size = val.parse().unwrap_or(DEFAULT_SMOOTH_SIZE);
                }
            }
            "--gain" | "-g" => {
                if let Some(val) = args.next() {
                    settings.gain = val.parse().unwrap_or(GAIN);
                }
            }
            "--fps" | "-f" => {
                if let Some(val) = args.next() {
                    settings.fps = val.parse().unwrap_or(FPS);
                }
            }
            "--color1" | "-c1" => {
                if let Some(val) = args.next() {
                    settings.color1 = color_from_string(&val);
                }
            }
            "--color2" | "-c2" => {
                if let Some(val) = args.next() {
                    settings.color2 = color_from_string(&val);
                }
            }
            "--color3" | "-c3" => {
                if let Some(val) = args.next() {
                    settings.color3 = color_from_string(&val);
                }
            }
            "--skew" | "-S" => {
                if let Some(val) = args.next() {
                    settings.skew = val.parse().unwrap_or(DEFAULT_SKEW);
                }
            }
            "--fft_size" | "-F" => {
                if let Some(val) = args.next() {
                    settings.fft_size = val.parse().unwrap_or(FFT_SIZE);
                    settings.set_fft_size(settings.fft_size);
                }
            }
            "--brightness" | "-b" => {
                if let Some(val) = args.next() {
                    settings.brightness = val.parse().unwrap_or(0.0);
                }
            }
            "--display_mode" | "-d" => {
                if let Some(val) = args.next() {
                    settings.display_mode = match val.as_str() {
                        "spectrum" => DisplayMode::Spectrum,
                        "oscilloscope" => DisplayMode::Oscilloscope,
                        "color_gradient" => DisplayMode::ColorGradient,
                        _ => DisplayMode::Spectrum,
                    };
                }
            }
            "--animation_mode" | "-a" => {
                if let Some(val) = args.next() {
                    settings.animation_mode = match val.as_str() {
                        "full" => AnimationMode::Full,
                        "full_with_max" => AnimationMode::FullWithMax,
                        "points" => AnimationMode::Points,
                        "full_middle" => AnimationMode::FullMiddle,
                        "full_middle_with_max" => AnimationMode::FullMiddleWithMax,
                        "points_middle" => AnimationMode::PointsMiddle,
                        _ => AnimationMode::Full,
                    };
                }
            }
            _ => {}
        }
    }

    settings
}

pub fn display_usage() {
    println!("Usage: audio_visualizer [OPTIONS]");
    println!("Options:");
    println!("  -s, --smooth <size>          Set the smooth size (default: {})", DEFAULT_SMOOTH_SIZE);
    println!("  -g, --gain <value>           Set the gain (default: {})", GAIN);
    println!("  -f, --fps <value>            Set the frames per second (default: {})", FPS);
    println!("  -c1, --color1 <color>        Set the first color (default: blue)");
    println!("  -c2, --color2 <color>        Set the second color (default: red)");
    println!("  -c3, --color3 <color>        Set the third color (default: magenta)");
    println!("  -S, --skew <value>           Set the skew value (default: {})", DEFAULT_SKEW);
    println!("  -F, --fft_size <size>        Set the FFT size (default: {})", FFT_SIZE);
    println!("  -b, --brightness <value>     Set the brightness (default: 1.0)");
    println!("  -d, --display_mode <mode>    Set the display mode (spectrum, oscilloscope, color_gradient; default: spectrum)");
    println!("  -a, --animation_mode <mode>  Set the animation mode (full, full_with_max, points, full_middle, full_middle_with_max, points_middle; default: full)");
}
