mod constants;

use crate::constants::*;
use std::{thread, time::Duration, io::Write, sync::{Arc, Mutex}};
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
const buffer_size: usize = 10; // Size of the rolling average buffer

fn main() -> anyhow::Result<()> {
    // --- Audio Setup ---
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no capture device found");

    // Print the device name
    println!("Using device: {}", device.name()?);
    let config: StreamConfig = device.default_input_config()?.into();

    let audio_level = Arc::new(Mutex::new(0.0f32));
    let audio_level_clone = audio_level.clone();

    let audio_levels = Arc::new(Mutex::new(vec![0.0f32; buffer_size])); // Rolling average buffer

    let audio_levels_clone = audio_levels.clone();

    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            process_audio_data(data, &audio_level_clone, &audio_levels_clone);
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

    loop {
        let current_audio_level = *audio_level.lock().unwrap();
        let num_leds_to_light = (current_audio_level * GAIN * NUM_LEDS as f32).min(NUM_LEDS as f32) as usize;
        let level254 = (current_audio_level * GAIN * 254.0).min(254.0) as u8;

        let mut buf = Vec::with_capacity(NUM_LEDS * 3 + 1);
        for i in 0..NUM_LEDS {
            if i < num_leds_to_light {
                // Lit LEDs - White
                buf.extend_from_slice(&[level254, 0, level254]);
            } else {
                // Off LEDs - Black
                buf.extend_from_slice(&[0, 0, 0]);
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
fn process_audio_data(data: &[f32], audio_level_clone: &Arc<Mutex<f32>>, audio_levels_clone: &Arc<Mutex<Vec<f32>>>) {

    // Update the rolling average buffer
    let mut audio_levels = audio_levels_clone.lock().unwrap();
    audio_levels.push(data.iter().map(|s| s.abs()).sum::<f32>() / data.len() as f32);
    if audio_levels.len() > buffer_size {
        audio_levels.remove(0);
    }

    // Calculate the average of the rolling average buffer
    let avg = audio_levels.iter().sum::<f32>() / audio_levels.len() as f32;
    *audio_level_clone.lock().unwrap() = avg;
}