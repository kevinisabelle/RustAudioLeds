mod constants;
mod color;

use crate::constants::*;
use std::{thread, time::Duration, io::Write, sync::{Arc, Mutex}};
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::color::{BLACK, RED};

const buffer_size: usize = 10; // Size of the rolling average buffer

struct Settings  {
    smooth_size: usize,
    gain: f32,
    fps: u64,
    color: String,
}

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

fn main() -> anyhow::Result<()> {
    // --- Audio Setup ---
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no capture device found");

    // Print the device name
    println!("Using device: {}", device.name()?);
    let config: StreamConfig = device.default_input_config()?.into();

    let mut settings = Settings {
        smooth_size: buffer_size,
        gain: GAIN,
        fps: FPS,
        color: String::from("white"),
    };

    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--smooth" => {
                if let Some(val) = args.next() {
                    settings.smooth_size = val.parse().unwrap_or(buffer_size);
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

    let audio_level = Arc::new(Mutex::new(0.0f32));
    let audio_level_clone = audio_level.clone();

    let audio_levels = Arc::new(Mutex::new(vec![0.0f32; settings_arc.smooth_size])); // Rolling average buffer

    let audio_levels_clone = audio_levels.clone();
    let settings_arc_clone = settings_arc.clone();

    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            process_audio_data(data, &audio_level_clone, &audio_levels_clone, &settings_arc_clone);
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None
    )?;
    input_stream.play()?;
    // --- End Audio Setup ---

    let mut port = serialport::new(PORT, BAUD)
        .timeout(Duration::from_millis(10))
        .open()?;

    let frame_delay = Duration::from_millis(1_000 / FPS);

    let color = color::color_from_string(&settings_arc.color);

    loop {
        let current_audio_level = *audio_level.lock().unwrap();
        let num_leds_to_light = (current_audio_level * settings_arc.gain * NUM_LEDS as f32).min(NUM_LEDS as f32) as usize;

        let mut buf = Vec::with_capacity(NUM_LEDS * 3 + 1);
        for i in 0..NUM_LEDS {
            if i < num_leds_to_light {
                // Lit LEDs - White
                buf.extend_from_slice(&color.brightness(current_audio_level).to_slice());
            } else {
                // Off LEDs - Black
                buf.extend_from_slice(&BLACK.to_slice());
            }
        }
        buf.push(END_MARKER); // terminate
        // ------------------------------------------------------------

        port.write_all(&buf)?;
        port.flush()?;                      // ensure everything is on the wire
        thread::sleep(frame_delay);
    }
}

/// Process the audio data and update the audio level
fn process_audio_data(data: &[f32], audio_level_clone: &Arc<Mutex<f32>>, audio_levels_clone: &Arc<Mutex<Vec<f32>>>, settings: &Arc<Settings>) {

    // Update the rolling average buffer
    let mut audio_levels = audio_levels_clone.lock().unwrap();
    audio_levels.push(data.iter().map(|s| s.abs()).sum::<f32>() / data.len() as f32);
    if audio_levels.len() > settings.smooth_size {
        audio_levels.remove(0);
    }

    // Calculate the average of the rolling average buffer
    let avg = audio_levels.iter().sum::<f32>() / audio_levels.len() as f32;
    *audio_level_clone.lock().unwrap() = avg;
}