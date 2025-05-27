use std::sync::{Arc, Mutex};
use crate::color::{color_from_string, Color};
use crate::DEFAULT_SMOOTH_SIZE;
use crate::constants::{DEFAULT_SKEW, FFT_SIZE, FPS, GAIN};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DisplayMode {
    Spectrum,
    Oscilloscope,
    ColorGradient,
}

#[derive(Debug)]
pub enum AnimationMode {
    Full,
    FullWithMax,
    Points,
    FullMiddle,
    FullMiddleWithMax,
    PointsMiddle,
}

#[derive(Debug)]
pub struct Settings  {
    pub smooth_size: usize,
    pub gain: f32,
    pub fps: u64,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color1_object: Color,
    pub color2_object: Color,
    pub color3_object: Color,
    pub fft_size: usize,
    pub frequencies:  Vec<f32>,
    pub gains: Vec<f32>,
    pub skew: f32,
    pub brightness: f32,
    pub display_mode: DisplayMode,
    pub animation_mode: AnimationMode,
}

pub struct SamplesWindow
{
    pub samples: Arc<Mutex<Vec<f32>>>,
    max_size: usize,
}

impl SamplesWindow {
    pub fn new(max_size: usize) -> Self {
        SamplesWindow {
            samples: Arc::new(Mutex::new(Vec::with_capacity(max_size))),
            max_size,
        }
    }

    pub fn add_sample(&mut self, sample: f32) {
        let mut samples = self.samples.lock().unwrap();
        if samples.len() >= self.max_size {
            samples.remove(0);
        }
        samples.push(sample);
    }

    pub fn add_samples(&mut self, samples: &[f32]) {
        for sample in samples {
            self.add_sample(*sample);
        }
    }

    pub fn average(&self) -> f32 {
        let samples = self.samples.lock().unwrap();
        if samples.is_empty() {
            0.0
        } else {
            samples.iter().copied().sum::<f32>() / samples.len() as f32
        }
    }

    pub fn max(&self) -> f32 {
        let samples = self.samples.lock().unwrap();
        if samples.is_empty() {
            0.0
        } else {
            *samples.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
        }
    }
}

pub type FrequenciesValues = Vec<SamplesWindow>;

pub fn get_config() -> Settings {

    let mut settings = Settings {
        smooth_size: DEFAULT_SMOOTH_SIZE,
        gain: GAIN,
        fps: FPS,
        color1: String::from("blue"),
        color2: String::from("red"),
        color3: String::from("magenta"),
        fft_size: FFT_SIZE,
        skew: DEFAULT_SKEW,
        brightness: 0.0,
        display_mode: DisplayMode::Spectrum,
        frequencies: vec![41.0, 55.0, 65.0, 82.0, 110.0, 146.0, 220.0, 261.0, 329.0, 392.0,
                          440.0, 523.0, 880.0, 987.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7500.0,
                          9000.0, 13000.0],
        gains: vec![1.3, 1.2, 1.1, 1.0, 1.0, 1.0, 1.0, 0.85, 0.75, 0.75,
                    0.75, 0.75, 0.75, 0.75, 1.0, 1.0, 1.0, 1.0, 1.2, 3.0,
                    4.0, 4.0],
        animation_mode: AnimationMode::Full,
        color1_object: color_from_string("blue"),
        color2_object: color_from_string("red"),
        color3_object: color_from_string("magenta"),
    };

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
                    settings.color1 = val;
                    settings.color1_object = color_from_string(&settings.color1);
                }
            }
            "--color2" | "-c2" => {
                if let Some(val) = args.next() {
                    settings.color2 = val;
                    settings.color2_object = color_from_string(&settings.color2);
                }
            }
            "--color3" | "-c3" => {
                if let Some(val) = args.next() {
                    settings.color3 = val;
                    settings.color3_object = color_from_string(&settings.color3);
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
