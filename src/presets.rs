use std::sync::MutexGuard;
use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::constants::NUM_LEDS;
use crate::settings::{AnimationMode, DisplayMode, Settings};

#[derive(Serialize, Deserialize, Debug)]
pub struct Preset {
    pub index: u8,
    pub name: [u8; 16],
    pub smooth_size: u16,
    pub gain: f32,
    pub fps: u16,
    pub color1: [u8; 3],
    pub color2: [u8; 3],
    pub color3: [u8; 3],
    pub fft_size: u16,
    pub frequencies: [f32; 22],
    pub gains: [f32; 22],
    pub skew: f32,
    pub brightness: f32,
    pub display_mode: DisplayMode, // enum encoded as u8
    pub animation_mode: AnimationMode, // enum encoded as u8
}

impl Preset {
    pub fn from_settings(settings: &Settings, index: u8, name: [u8; 16]) -> Preset {
        Preset {
            index,
            name,
            smooth_size: settings.smooth_size as u16,
            gain: settings.gain,
            fps: settings.fps as u16,
            color1: settings.color1.to_slice(),
            color2: settings.color2.to_slice(),
            color3: settings.color3.to_slice(),
            fft_size: settings.fft_size as u16,
            frequencies: settings.frequencies.clone().try_into().unwrap_or([0.0; 22]),
            gains: settings.gains.clone().try_into().unwrap_or([0.0; 22]),
            skew: settings.skew,
            brightness: settings.brightness,
            display_mode: settings.display_mode.clone(),
            animation_mode: settings.animation_mode.clone(),
        }
    }

    pub fn to_settings(&self) -> Settings {
        let mut settings = Settings {
            smooth_size: self.smooth_size as usize,
            gain: self.gain,
            fps: self.fps as usize,
            color1: Color::from_slice(&self.color1),
            color2: Color::from_slice(&self.color2),
            color3: Color::from_slice(&self.color3),
            fft_size: self.fft_size as usize,
            frequencies: self.frequencies.to_vec(),
            gains: self.gains.to_vec(),
            skew: self.skew,
            brightness: self.brightness,
            display_mode: self.display_mode.clone(),
            animation_mode: self.animation_mode.clone(),
            led_buffer: vec![0; NUM_LEDS * 3 + 1], // Assuming 22 frequencies, each with RGB values
            cached_df: 0.0, // Set by `set_fft_size`
            selected_preset: 0, // Default value, can be set later
            active_preset: self.index as usize,
        };
        settings.set_fft_size(self.fft_size as usize);
        settings
    }

    pub fn apply_to_settings(&self, settings: &mut MutexGuard<Settings>) {
        settings.smooth_size = self.smooth_size as usize;
        settings.gain = self.gain;
        settings.fps = self.fps as usize;
        settings.color1 = Color::from_slice(&self.color1);
        settings.color2 = Color::from_slice(&self.color2);
        settings.color3 = Color::from_slice(&self.color3);
        settings.fft_size = self.fft_size as usize;
        settings.frequencies = self.frequencies.to_vec();
        settings.gains = self.gains.to_vec();
        settings.skew = self.skew;
        settings.brightness = self.brightness;
        settings.display_mode = self.display_mode.clone();
        settings.animation_mode = self.animation_mode.clone();
        settings.active_preset = self.index as usize;
    }
}

const PRESET_PATH: &str = "presets";

pub fn save_preset(preset: &Preset) -> std::io::Result<()> {
    let preset_index = preset.index;
    let preset_filename = format!("{}/preset_{}.bin", PRESET_PATH, preset_index);
    let encoded = postcard::to_stdvec(&preset).unwrap();
    std::fs::write(preset_filename, encoded)?;
    Ok(())
}

pub fn load_preset(index: u8) -> std::io::Result<Preset> {
    let preset_filename = format!("{}/preset_{}.bin", PRESET_PATH, index);
    let data = std::fs::read(preset_filename)?;
    let preset: Preset = postcard::from_bytes(&data).unwrap();
    Ok(preset)
}

pub fn list_presets() -> std::io::Result<Vec<Preset>> {
    let mut presets = Vec::new();
    for entry in std::fs::read_dir(PRESET_PATH)? {
        let entry = entry?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("bin") {
            let preset_index = entry.file_name().to_string_lossy()
                .replace("preset_", "")
                .replace(".bin", "");
            if let Ok(index) = preset_index.parse::<u8>() {
                if let Ok(preset) = load_preset(index) {
                    presets.push(preset);
                }
            }
        }
    }
    Ok(presets)
}

pub fn delete_preset(index: u8) -> std::io::Result<()> {
    let preset_filename = format!("{}/preset_{}.bin", PRESET_PATH, index);
    std::fs::remove_file(preset_filename)?;
    Ok(())
}

