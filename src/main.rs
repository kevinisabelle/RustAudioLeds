use std::{thread, time::Duration, io::Write, sync::{Arc, Mutex}};
use cpal::StreamConfig;
use serialport::SerialPort;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// ---------- parameters kept in sync with the Arduino sketch ----------
const NUM_LEDS: usize = 12 * 22;
const END_MARKER: u8 = 0xFF;
const BAUD: u32 = 500_000;
const PORT: &str = "/dev/ttyUSB0";        // adapt to your system
const FPS: u64 = 60;
const GAIN: f32 = 1.0; // Adjust this to change sensitivity to audio level
// ---------------------------------------------------------------------

fn main() -> anyhow::Result<()> {
    // --- Audio Setup ---
    let host = cpal::default_host();

    println!("Available input devices:");
    for device in host.input_devices()? {
        println!("  {}", device.name()?);
    }

    let device = host.default_input_device().expect("no capture device found");

    // Print the device name
    println!("Using device: {}", device.name()?);
    let config: StreamConfig = device.default_input_config()?.into();

    let audio_level = Arc::new(Mutex::new(0.0f32));
    let audio_level_clone = audio_level.clone();

    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            let avg = data.iter().map(|s| s.abs()).sum::<f32>() / data.len() as f32;
            // Print number of samples
            // println!("Number of samples: {}", data.len());
            *audio_level_clone.lock().unwrap() = avg;
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

        // Print the current audio level
        // println!("Current audio level: {}", current_audio_level);

        // Calculate the number of LEDs to light up
        let num_leds_to_light = (current_audio_level * GAIN * NUM_LEDS as f32).min(NUM_LEDS as f32) as usize;

        // Print the number of LEDs to light up
        // println!("Number of LEDs to light up: {}", num_leds_to_light);
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

// ---------- tiny HSV→RGB helper (8-bit fast & good enough) ----------
fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    if s == 0 {
        return (v, v, v);
    }
    let region = h / 43;
    let remainder = (h as u16 - (region as u16 * 43)) * 6;

    let p = (v as u16 * (255 - s as u16)) / 255;
    let q = (v as u16 * (255 - ((s as u16 * remainder) / 255))) / 255;
    let t = (v as u16 * (255 - ((s as u16 * (255 - remainder)) / 255))) / 255;

    match region {
        0 => (v, t as u8, p as u8),
        1 => (q as u8, v, p as u8),
        2 => (p as u8, v, t as u8),
        3 => (p as u8, q as u8, v),
        4 => (t as u8, p as u8, v),
        _ => (v, p as u8, q as u8),
    }
}

