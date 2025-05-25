use std::sync::{Arc, Mutex};
use crate::DEFAULT_SMOOTH_SIZE;
use crate::constants::{DEFAULT_SKEW, FFT_SIZE, FPS, GAIN};

pub enum DisplayMode {
    Spectrum,
    Oscilloscope,
    ColorGradient,
}

pub enum AnimationMode {
    Full,
    FullWithMax,
    Points,
    FullMiddle,
    FullMiddleWithMax,
    PointsMiddle,
}

pub struct Settings  {
    pub smooth_size: usize,
    pub gain: f32,
    pub fps: u64,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub fft_size: usize,
    pub frequencies:  Vec<f32>,
    pub gains: Vec<f32>,
    pub skew: f32,
}

impl std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("smooth_size", &self.smooth_size)
            .field("gain", &self.gain)
            .field("fps", &self.fps)
            .field("color1", &self.color1)
            .field("color2", &self.color2)
            .field("color3", &self.color3)
            .field("fft_size", &self.fft_size)
            .field("frequencies", &self.frequencies)
            .field("gains", &self.gains)
            .field("skew", &self.skew)
            .finish()
    }
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
        frequencies: vec![41.0, 55.0, 65.0, 82.0, 110.0, 146.0, 220.0, 261.0, 329.0, 392.0,
                          440.0, 523.0, 880.0, 987.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7500.0,
                          9000.0, 13000.0],
        gains: vec![1.3, 1.2, 1.1, 1.0, 1.0, 1.0, 1.0, 0.85, 0.75, 0.75,
                    0.75, 0.75, 0.75, 0.75, 1.0, 1.0, 1.0, 1.0, 1.2, 3.0,
                    4.0, 4.0],
    };

    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--smooth" => {
                if let Some(val) = args.next() {
                    settings.smooth_size = val.parse().unwrap_or(DEFAULT_SMOOTH_SIZE);
                }
            }
            "--gain" => {
                if let Some(val) = args.next() {
                    settings.gain = val.parse().unwrap_or(GAIN);
                }
            }
            "--fps" => {
                if let Some(val) = args.next() {
                    settings.fps = val.parse().unwrap_or(FPS);
                }
            }
            "--color1" => {
                if let Some(val) = args.next() {
                    settings.color1 = val;
                }
            }
            "--color2" => {
                if let Some(val) = args.next() {
                    settings.color2 = val;
                }
            }
            "--color3" => {
                if let Some(val) = args.next() {
                    settings.color3 = val;
                }
            }
            "--skew" => {
                if let Some(val) = args.next() {
                    settings.skew = val.parse().unwrap_or(DEFAULT_SKEW);
                }
            }
            "--fft_size" => {
                if let Some(val) = args.next() {
                    settings.fft_size = val.parse().unwrap_or(FFT_SIZE);
                }
            }
            _ => {}
        }
    }

    settings
}
