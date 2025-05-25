mod constants;
mod color;
mod settings;
mod animations;
mod dsp;

use crate::animations::{full_spectrum, full_spectrum_with_max, points_spectrum, spectrum_middle, spectrum_middle_with_max};
use crate::color::{Color, BLACK};
use crate::constants::*;
use crate::dsp::{half_window_bins, process_audio_data};
use crate::settings::{display_usage, get_config, AnimationMode, DisplayMode, FrequenciesValues, SamplesWindow, Settings};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use thread::sleep;

fn main() -> anyhow::Result<()> {

    display_usage();
    
    println!("Starting LED Strip Visualizer...");

    // --- Configuration ---
    let settings = get_config();
    let settings_arc = Arc::new(settings);
    let settings_arc_clone = settings_arc.clone();

    let samples_window = SamplesWindow::new(settings_arc.fft_size);
    let samples_window_arc = Arc::new(Mutex::new(samples_window));

    let df = SAMPLE_RATE as f32 / settings_arc.fft_size as f32; // frequency resolution (Hz)

    // --- Print settings ---
    println!("Current Settings: {:?}", settings_arc);
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

    let mut buf = Vec::with_capacity(NUM_LEDS * 3 + 1);
    let nb_frequency_levels = frequency_levels.lock().unwrap().len();

    for i in 0..nb_frequency_levels {
        let level = frequency_levels.lock().unwrap()[i].average();
        let max = frequency_levels.lock().unwrap()[i].max();
        let strip_colors = get_strip_colors(level, max, settings_arc, i);
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

fn get_strip_colors(level: f32, max: f32, settings_arc: &Arc<Settings>, index: usize) -> Vec<Color> {

    let mut strip_colors = vec![BLACK; LEDS_PER_STRIP];
    let freq_gain = settings_arc.gains[index];
    let level_adjusted = level * settings_arc.gain * freq_gain;
    let max_adjusted = max * settings_arc.gain * freq_gain;

    match settings_arc.display_mode
    {
        DisplayMode::Spectrum => {
            
            match settings_arc.animation_mode
            {
                AnimationMode::Full =>
                    {
                        full_spectrum(level_adjusted, index, settings_arc, &mut strip_colors);
                    }
                AnimationMode::FullWithMax =>
                    {
                        full_spectrum_with_max(level_adjusted, max_adjusted, index, settings_arc, &mut strip_colors);
                    }
                AnimationMode::Points =>
                    {
                        points_spectrum(level_adjusted, index, settings_arc, &mut strip_colors);
                    }
                AnimationMode::FullMiddle =>
                    {
                        spectrum_middle(level_adjusted, index, settings_arc, &mut strip_colors);
                    }
                AnimationMode::FullMiddleWithMax =>
                    {
                        spectrum_middle_with_max(level_adjusted, max_adjusted, index, settings_arc, &mut strip_colors);
                    }
                _ => {
                    full_spectrum(level_adjusted, index, settings_arc, &mut strip_colors);
                }
            }
        }
        DisplayMode::Oscilloscope => {
            
        }
        DisplayMode::ColorGradient => {
            for i in 0..LEDS_PER_STRIP {
                let mix_factor = (i+1) as f32 / LEDS_PER_STRIP as f32;
                let color = settings_arc.color1_object.clone().mix(&settings_arc.color2_object.clone(), mix_factor);
                strip_colors[i] = color.clone();
            }
        }
    }

    strip_colors
}