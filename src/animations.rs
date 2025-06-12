use crate::color::{Color, BLACK};
use crate::constants::{END_MARKER, LEDS_PER_STRIP, NUM_LEDS};
use crate::settings::{AnimationMode, DisplayMode, Settings};
use crate::values::{SamplesWindow, StateValues};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn animate_leds(state_values: &Arc<Mutex<StateValues>>, settings_arc: &Arc<Mutex<Settings>>, port: &mut dyn serialport::SerialPort) {

    let frequency_levels = state_values.lock().unwrap().frequencies.clone();
    let settings = settings_arc.lock().unwrap().clone();
    let frame_delay = Duration::from_millis(1_000 / settings.fps as u64);

    let mut buf = Vec::with_capacity(NUM_LEDS * 3 + 1);
    let nb_frequency_levels = settings_arc.lock().unwrap().frequencies.len();
    let sample_to_average = settings_arc.lock().unwrap().smooth_size;

    if (settings.display_mode == DisplayMode::Oscilloscope){

    } else {
        for freq in 0..nb_frequency_levels {
            let level = frequency_levels[freq].average(sample_to_average);
            // println!("Frequency {}: Level: {:.3}", freq, level);
            let max = frequency_levels[freq].max(sample_to_average);
            let strip_colors = get_strip_colors(level, max, &settings.clone(), freq);
            output_colors_to_buffer(&mut buf, &strip_colors, freq);
        }
    }

    buf.push(END_MARKER);
    settings_arc.lock().unwrap().led_buffer[..buf.len()].copy_from_slice(&buf);

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

fn get_strip_colors(level: f32, max: f32, settings: &Settings, index: usize) -> Vec<Color> {

    let mut strip_colors = vec![BLACK; LEDS_PER_STRIP];
    let freq_gain = settings.gains[index];
    let level_adjusted = level * settings.gain * freq_gain;
    let max_adjusted = max * settings.gain * freq_gain;

    match settings.display_mode
    {
        DisplayMode::Spectrum => {

            match settings.animation_mode
            {
                AnimationMode::Full =>
                    {
                        full_spectrum(level_adjusted, index, settings, &mut strip_colors);
                    }
                AnimationMode::FullWithMax =>
                    {
                        full_spectrum_with_max(level_adjusted, max_adjusted, index, settings, &mut strip_colors);
                    }
                AnimationMode::Points =>
                    {
                        points_spectrum(level_adjusted, index, settings, &mut strip_colors);
                    }
                AnimationMode::FullMiddle =>
                    {
                        spectrum_middle(level_adjusted, index, settings, &mut strip_colors);
                    }
                AnimationMode::FullMiddleWithMax =>
                    {
                        spectrum_middle_with_max(level_adjusted, max_adjusted, index, settings, &mut strip_colors);
                    }
                _ => {
                    full_spectrum(level_adjusted, index, settings, &mut strip_colors);
                }
            }
        }
        DisplayMode::Oscilloscope => {

        }
        DisplayMode::ColorGradient => {
            for i in 0..LEDS_PER_STRIP {
                let mix_factor = (i+1) as f32 / LEDS_PER_STRIP as f32;
                let color = settings.color1.clone().mix(&settings.color2.clone(), mix_factor);
                strip_colors[i] = color.clone();
            }
        }
    }

    strip_colors
}

pub fn full_spectrum(
    level: f32,
    index: usize,
    settings_arc: &Settings,
    strip_colors: &mut Vec<Color>,
) {
    let color1 = settings_arc.color1.clone();
    let color2 = settings_arc.color2.clone();
    let num_leds_to_light_float = (level * LEDS_PER_STRIP as f32).min(LEDS_PER_STRIP as f32);
    let num_leds_to_light = num_leds_to_light_float.ceil() as usize;
    let leftover_value = 1.0 - (num_leds_to_light as f32 - num_leds_to_light_float).max(0.0);
    for i in 0..num_leds_to_light {
        let mix_factor = (i+1) as f32 / num_leds_to_light as f32;
        let color = color1.mix(&color2, mix_factor);
        strip_colors[i] = color.clone(); //.brightness(1.0 - (i as f32 / num_leds_to_light as f32));

        if num_leds_to_light == 1 && i == 0 {
            strip_colors[i] = color2.mix(&color1.clone(), leftover_value).clone();
        }

        if i == num_leds_to_light - 1 {
            strip_colors[i] = strip_colors[i].brightness(leftover_value);
        }
    }
}

pub fn full_spectrum_with_max(
    level: f32,
    max: f32,
    index: usize,
    settings_arc: &Settings,
    strip_colors: &mut Vec<Color>,
) {
    full_spectrum(level, index, settings_arc, strip_colors);
    strip_colors[LEDS_PER_STRIP-1] = settings_arc.color3.clone().brightness(level.min(1.0));
}

pub fn points_spectrum(
    level: f32,
    index: usize,
    settings_arc: &Settings,
    strip_colors: &mut Vec<Color>,
) {
    let mut first_led_index = (level * LEDS_PER_STRIP as f32).floor() as usize;
    let last_led_index = (level * LEDS_PER_STRIP as f32).ceil() as usize;
    let factor = (level * LEDS_PER_STRIP as f32) - first_led_index as f32;

    first_led_index = first_led_index.min(LEDS_PER_STRIP - 1);
    let color_to_use = settings_arc.color1.clone().mix(&settings_arc.color2.clone(), level);

    strip_colors[first_led_index] = color_to_use.brightness(1.0 - factor);
    if last_led_index < LEDS_PER_STRIP {
        strip_colors[last_led_index] = color_to_use.brightness(factor);
    }
}

pub fn spectrum_middle(
    level: f32,
    index: usize,
    settings_arc: &Settings,
    strip_colors: &mut Vec<Color>,
) {
    let color1 = settings_arc.color1.clone();
    let color2 = settings_arc.color2.clone();
    let middle_index = LEDS_PER_STRIP / 2;

    let num_leds_to_light_float = (level * middle_index as f32).min(middle_index as f32);
    let num_leds_to_light = num_leds_to_light_float.ceil() as usize;

    let leftover_value = 1.0 - (num_leds_to_light as f32 - num_leds_to_light_float).max(0.0);

    for i in 0..num_leds_to_light {
        let mix_factor = (i + 1) as f32 / num_leds_to_light as f32;
        let color = color1.mix(&color2, mix_factor);
        strip_colors[middle_index - i - 1] = color.clone();
        strip_colors[middle_index + i] = color.clone();

        if num_leds_to_light == 1 && i == 0 {
            strip_colors[middle_index - i - 1] = color2.mix(&color1.clone(), leftover_value).clone();
            strip_colors[middle_index + i] = strip_colors[middle_index - i - 1].clone();
        }

        if i == num_leds_to_light - 1 {
            strip_colors[middle_index - i - 1] = strip_colors[middle_index - i - 1].brightness(leftover_value);
            strip_colors[middle_index + i] = strip_colors[middle_index + i].brightness(leftover_value);
        }
    }
}

pub fn spectrum_middle_with_max(
    level: f32,
    max: f32,
    index: usize,
    settings_arc: &Settings,
    strip_colors: &mut Vec<Color>,
) {
    spectrum_middle(level, index, settings_arc, strip_colors);
    strip_colors[LEDS_PER_STRIP-1] = settings_arc.color3.clone().brightness(level.min(1.0));
    strip_colors[0] = settings_arc.color3.clone().brightness(level.min(1.0));
}