use std::sync::Arc;
use crate::color::{color_from_string, Color};
use crate::constants::LEDS_PER_STRIP;
use crate::settings::Settings;

pub fn full_spectrum(
    level: f32,
    index: usize,
    settings_arc: &Arc<Settings>,
    strip_colors: &mut Vec<Color>,
) {
    let color1 = settings_arc.color1_object.clone();
    let color2 = settings_arc.color2_object.clone();
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
    settings_arc: &Arc<Settings>,
    strip_colors: &mut Vec<Color>,
) {
    full_spectrum(level, index, settings_arc, strip_colors);
    strip_colors[LEDS_PER_STRIP-1] = settings_arc.color3_object.clone().brightness(level.min(1.0));
}

pub fn points_spectrum(
    level: f32,
    index: usize,
    settings_arc: &Arc<Settings>,
    strip_colors: &mut Vec<Color>,
) {
    let mut first_led_index = (level * LEDS_PER_STRIP as f32).floor() as usize;
    let last_led_index = (level * LEDS_PER_STRIP as f32).ceil() as usize;
    let factor = (level * LEDS_PER_STRIP as f32) - first_led_index as f32;

    first_led_index = first_led_index.min(LEDS_PER_STRIP - 1);
    let color_to_use = settings_arc.color1_object.clone().mix(&settings_arc.color2_object.clone(), level);

    strip_colors[first_led_index] = color_to_use.brightness(1.0 - factor);
    if last_led_index < LEDS_PER_STRIP {
        strip_colors[last_led_index] = color_to_use.brightness(factor);
    }
}

pub fn spectrum_middle(
    level: f32,
    index: usize,
    settings_arc: &Arc<Settings>,
    strip_colors: &mut Vec<Color>,
) {
    let color1 = settings_arc.color1_object.clone();
    let color2 = settings_arc.color2_object.clone();
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
    settings_arc: &Arc<Settings>,
    strip_colors: &mut Vec<Color>,
) {
    spectrum_middle(level, index, settings_arc, strip_colors);
    strip_colors[LEDS_PER_STRIP-1] = settings_arc.color3_object.clone().brightness(level.min(1.0));
    strip_colors[0] = settings_arc.color3_object.clone().brightness(level.min(1.0));
}