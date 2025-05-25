mod constants;
mod color;
mod settings;

use crate::constants::*;
use std::{thread, time::Duration, sync::{Arc, Mutex}};
use thread::sleep;
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use crate::color::{Color, BLACK};
use crate::settings::{get_config, FrequenciesValues, SamplesWindow, Settings};

fn main() -> anyhow::Result<()> {

    println!("Starting LED Strip Visualizer...");

    // --- Configuration ---
    let settings = get_config();
    let settings_arc = Arc::new(settings);
    let settings_arc_clone = settings_arc.clone();

    let samples_window = SamplesWindow::new(settings_arc.fft_size);
    let samples_window_arc = Arc::new(Mutex::new(samples_window));

    let df = SAMPLE_RATE as f32 / settings_arc.fft_size as f32; // frequency resolution (Hz)

    // --- Print settings ---
    println!("Settings: {:?}", settings_arc);
    println!("Frequency resolution: {} Hz", df);

    for f in &settings_arc.frequencies {
        let bins = half_window_bins(*f, df);
        println!("Frequency: {} Hz, Bins: {}", f, bins);
    }

    let mut frequencies = FrequenciesValues::new();
    for _ in 0..settings_arc.frequencies.len() {
        frequencies.push(SamplesWindow::new(settings_arc.smooth_size));
    }
    let frequencies_arc = Arc::new(Mutex::new(frequencies));
    let frequencies_arc_clone = frequencies_arc.clone();

    // --- Audio Setup ---
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no capture device found");

    // Print the device name
    println!("Using device: {}", device.name()?);
    let config: StreamConfig = device.default_input_config()?.into();
    println!("Default input config: {:?}", config);

    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            process_audio_data(data, &frequencies_arc_clone, &settings_arc_clone, &samples_window_arc, df.clone());
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None
    )?;
    input_stream.play()?;

    // --- Serial Setup ---
    let mut port = serialport::new(PORT, BAUD)
        .timeout(Duration::from_millis(10))
        .open()?;

    // --- Render Loop ---
    loop {
        animate_leds(&frequencies_arc, &settings_arc, port.as_mut());
    }
}

fn animate_leds(frequency_levels: &Arc<Mutex<FrequenciesValues>>, settings_arc: &Arc<Settings>, port: &mut dyn serialport::SerialPort) {
    let frame_delay = Duration::from_millis(1_000 / settings_arc.fps);
    let color1 = color::color_from_string(&settings_arc.color1);
    let color2 = color::color_from_string(&settings_arc.color2);
    let color3 = color::color_from_string(&settings_arc.color3);

    let mut buf = Vec::with_capacity(NUM_LEDS * 3 + 1);
    let nb_frequency_levels = frequency_levels.lock().unwrap().len();

    for i in 0..nb_frequency_levels {
        let level = frequency_levels.lock().unwrap()[i].average();
        let max = frequency_levels.lock().unwrap()[i].max();
        let strip_colors = get_strip_colors(level, max, &color1, &color2, &color3, settings_arc, i);
        output_colors_to_buffer(&mut buf, &strip_colors, i);
    }

    buf.push(END_MARKER);

    port.write_all(&buf).unwrap();
    port.flush().unwrap();

    sleep(frame_delay);
}

fn output_colors_to_buffer(buf: &mut Vec<u8>, colors: &Vec<Color>, index: usize) {
    let is_reversed = index % 2 == 1;

    let ordered_colors = if is_reversed {
        colors.iter().rev().cloned().collect::<Vec<_>>()
    } else {
        colors.to_vec()
    };

    for color in ordered_colors {
        buf.extend_from_slice(&color.to_slice());
    }
}

fn get_strip_colors(level: f32, max: f32, color1: &Color, color2: &Color, color3: &Color, settings_arc: &Arc<Settings>, index: usize) -> Vec<Color> {

    let mut strip_colors = vec![BLACK; LEDS_PER_STRIP];
    let freq_gain = settings_arc.gains[index];
    let num_leds_to_light_float = (level * settings_arc.gain * freq_gain * LEDS_PER_STRIP as f32).min(LEDS_PER_STRIP as f32);
    let num_leds_to_light = num_leds_to_light_float.ceil() as usize;
    let leftover_value = 1.0 - (num_leds_to_light as f32 - num_leds_to_light_float).max(0.0);

    for i in 0..num_leds_to_light {
        let mix_factor = (i+1) as f32 / num_leds_to_light as f32;
        let color = color1.mix(color2, mix_factor);
        strip_colors[i] = color.clone(); //.brightness(1.0 - (i as f32 / num_leds_to_light as f32));

        if num_leds_to_light == 1 && i == 0 {
            strip_colors[i] = color2.mix(color1, leftover_value).clone();
        }

        if i == num_leds_to_light - 1 {
            strip_colors[i] = strip_colors[i].brightness(leftover_value);
        }
    }

    strip_colors
}

/// Process the audio data and update the audio level
fn process_audio_data(
    data: &[f32],
    frequency_levels: &Arc<Mutex<FrequenciesValues>>,
    settings: &Arc<Settings>,
    samples_window: &Arc<Mutex<SamplesWindow>>,
    df: f32,
) {
    // 1.  Move samples into the rolling window
    {
        let mut win = samples_window.lock().unwrap();
        win.add_samples(data);
        if win.samples.lock().unwrap().len() < settings.fft_size {
            return;                       // not enough for one FFT yet
        }
    }

    // 2.  FFT → linear magnitude spectrum (already √N-normalised)
    let spec = samples_fft_to_spectrum(
        &samples_window.lock().unwrap().samples.lock().unwrap(),
        SAMPLE_RATE,
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
    ).expect("FFT");

    for (i, &f_cfg) in settings.frequencies.iter().enumerate() {
        let idx_c = (f_cfg / df).round() as isize;          // centre bin
        let r     = half_window_bins(f_cfg, df);            // window radius

        let mut acc = 0.0;
        let mut n   = 0;
        for off in -(r as isize)..=(r as isize) {
            let idx = idx_c + off;
            if idx >= 0 && (idx as usize) < spec.data().len() {
                acc += spec.data()[idx as usize].1.val();
                n   += 1;
            }
        }
        let mut v = if n > 0 { acc / n as f32 } else { 0.0 };
        v *= weight(f_cfg, settings.skew);                           // high-freq boost
        frequency_levels.lock().unwrap()[i].add_sample(v);  // smooth between frames
    }
}

/// Exponential-law weighting.
/// alpha=0.0  → flat,   alpha>0 → boost highs,   alpha<0 → boost lows.
/// Typical values: alpha = 0.35 … 0.55 gives a gentle but audible lift of everything above ~1 kHz.
fn weight(freq_hz: f32, alpha: f32) -> f32 {
    let F_MIN: f32 = 0.0; // min frequency
    let F_MAX: f32 = SAMPLE_RATE as f32 / 2.0; // Nyquist frequency

    // normalised [0,1] then exponential
    let x = ((freq_hz - F_MIN) / (F_MAX - F_MIN)).clamp(0.0, 1.0);
    x.powf(alpha)
}

/// How many neighbouring bins to average on *each* side of the centre bin.
/// 1 ≈ 3-bin window, 2 ≈ 5-bin window, etc.
fn half_window_bins(freq_hz: f32, df: f32) -> usize {
    // keep ~15 % fractional bandwidth (tweak to taste)
    let bw_hz = 0.15 * freq_hz;
    ((bw_hz / df).round() as isize).max(1) as usize
}