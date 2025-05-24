mod constants;
mod color;

use crate::constants::*;
use std::{thread, time::Duration, io::Write, sync::{Arc, Mutex}};
use thread::sleep;
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use crate::color::{BLACK, RED};

const BUFFER_SIZE: usize = 10; // Size of the rolling average buffer

struct Settings  {
    smooth_size: usize,
    gain: f32,
    fps: u64,
    color: String,
    fft_size: usize,
    frequencies:  Vec<f32>
}

struct SamplesWindow
{
    samples: Arc<Mutex<Vec<f32>>>,
    max_size: usize,
}

impl SamplesWindow {
    fn new(max_size: usize) -> Self {
        SamplesWindow {
            samples: Arc::new(Mutex::new(Vec::with_capacity(max_size))),
            max_size,
        }
    }

    fn add_sample(&mut self, sample: f32) {
        let mut samples = self.samples.lock().unwrap();
        if samples.len() >= self.max_size {
            samples.remove(0);
        }
        samples.push(sample);
    }

    fn add_samples(&mut self, samples: &[f32]) {
        for sample in samples {
            self.add_sample(*sample);
        }
    }

    fn average(&self) -> f32 {
        let samples = self.samples.lock().unwrap();
        if samples.is_empty() {
            0.0
        } else {
            samples.iter().copied().sum::<f32>() / samples.len() as f32
        }
    }
}

type FrequenciesValues = Vec<SamplesWindow>;

impl std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("smooth_size", &self.smooth_size)
            .field("gain", &self.gain)
            .field("fps", &self.fps)
            .field("color", &self.color)
            .finish()
    }
}

struct SpectrumConfig {

}

fn main() -> anyhow::Result<()> {
    // --- Audio Setup ---
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no capture device found");

    // Print the device name
    println!("Using device: {}", device.name()?);
    let config: StreamConfig = device.default_input_config()?.into();

    let mut settings = Settings {
        smooth_size: BUFFER_SIZE,
        gain: GAIN,
        fps: FPS,
        color: String::from("white"),
        fft_size: FFT_SIZE,
        frequencies: vec![47.0, 60.0, 80.0, 100.0, 120.0, 180.0, 240.0, 320.0, 380.0, 480.0, 620.0, 780.0, 960.0,
                          1920.0, 3000.0, 4800.0, 6000.0, 8000.0, 9600.0, 12000.0, 14000.0, 19200.0]
    };

    let samples_window = SamplesWindow::new(settings.fft_size);
    let samples_window_arc = Arc::new(Mutex::new(samples_window));

    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--smooth" => {
                if let Some(val) = args.next() {
                    settings.smooth_size = val.parse().unwrap_or(BUFFER_SIZE);
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
            "--color" => {
                if let Some(val) = args.next() {
                    settings.color = val;
                }
            }
            _ => {}
        }
    }

    println!("Settings: {:?}", settings);

    let settings_arc = Arc::new(settings);
    let settings_arc_clone = settings_arc.clone();

    // let audio_levels = Arc::new(Mutex::new(vec![0.0f32; NUM_STRIPS])); // Rolling average buffer
    // let audio_levels_clone = audio_levels.clone();

    let mut frequencies = FrequenciesValues::new();
    for _ in 0..settings_arc.frequencies.len() {
        frequencies.push(SamplesWindow::new(settings_arc.smooth_size));
    }

    let frequencies_arc = Arc::new(Mutex::new(frequencies));
    let frequencies_arc_clone = frequencies_arc.clone();

    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            process_audio_data(data, &frequencies_arc_clone, &settings_arc_clone, &samples_window_arc);
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None
    )?;
    input_stream.play()?;
    // --- End Audio Setup ---

    let mut port = serialport::new(PORT, BAUD)
        .timeout(Duration::from_millis(10))
        .open()?;

    loop {
        animate_leds(&frequencies_arc, &settings_arc, port.as_mut());
    }
}

fn animate_leds(frequency_levels: &Arc<Mutex<FrequenciesValues>>, settings_arc: &Arc<Settings>, port: &mut dyn serialport::SerialPort) {
    let frame_delay = Duration::from_millis(1_000 / settings_arc.fps);
    let color = color::color_from_string(&settings_arc.color);

    let mut buf = Vec::with_capacity(NUM_LEDS * 3 + 1);
    let nb_frequency_levels = frequency_levels.lock().unwrap().len();

    for i in 0..nb_frequency_levels {
        let mut level = frequency_levels.lock().unwrap()[i].average();
        // level = level.min(1.0); // Clamp to 1.0
        let num_leds_to_light = (level * settings_arc.gain * LEDS_PER_STRIP as f32).min(LEDS_PER_STRIP as f32) as usize;
        // let color = color.brightness(level);

        if i%2 == 0 {
            for j in 0..LEDS_PER_STRIP {
                if j < num_leds_to_light {
                    // Lit LEDs - White
                    buf.extend_from_slice(&color.to_slice());
                } else {
                    // Off LEDs - Black
                    buf.extend_from_slice(&BLACK.to_slice());
                }
            }
        } else {
            for j in 0..LEDS_PER_STRIP {
                if j >= (LEDS_PER_STRIP - num_leds_to_light) {
                    // Lit LEDs - White
                    buf.extend_from_slice(&color.to_slice());
                } else {
                    // Off LEDs - Black
                    buf.extend_from_slice(&BLACK.to_slice());
                }
            }
        }
    }

    buf.push(END_MARKER); // terminate

    port.write_all(&buf).unwrap();
    port.flush().unwrap(); // ensure everything is on the wire
    sleep(frame_delay);
}

/// Process the audio data and update the audio level
fn process_audio_data(data: &[f32], frequency_levels: &Arc<Mutex<FrequenciesValues>>, settings: &Arc<Settings>, samples_window: &Arc<Mutex<SamplesWindow>>) {

    let mut samples_window = samples_window.lock().unwrap();
    samples_window.add_samples(data);

    if data.len() < settings.fft_size {
        return;
    }

    let res = samples_fft_to_spectrum(
        &samples_window.samples.lock().unwrap(),
        44100,
        FrequencyLimit::Range(20.0, 20000.0),
        Some(&divide_by_N_sqrt),
    ).unwrap();

    for i in 0..settings.frequencies.len() {
        let freq = settings.frequencies[i];
        let freq_val = res.freq_val_closest(freq).1.val();
        // println!("Freq: {}, Value: {}", freq, freq_val);
        frequency_levels.lock().unwrap()[i].add_sample(freq_val);
    }
}