use std::io::{Read, Write};
use std::sync::MutexGuard;
use crate::color::Color;
use crate::constants::NUM_LEDS;
use crate::settings::{AnimationMode, DisplayMode, Settings};

#[derive(Debug)]
pub struct Preset {
    pub index: u8, // Index 0
    pub name: [u8; 16], // Index 1-16
    pub smooth_size: u16, // Index 17-18
    pub gain: f32, // Index 19-22
    pub fps: u16, // Index 23-24
    pub color1: [u8; 3], // Index 25-27
    pub color2: [u8; 3], // Index 28-30
    pub color3: [u8; 3], // Index 31-33
    pub fft_size: u16, // Index 34-35
    pub frequencies: [f32; 22], // Number of bytes each frequency has 4 bytes, total 88 bytes for 22 frequencies, Index 36-99
    pub gains: [f32; 22], // Number of bytes each gain has 4 bytes, total 88 bytes for 22 gains, Index 100-163
    pub skew: f32, // Index 164-167
    pub brightness: f32, // Index 168-171
    pub display_mode: DisplayMode, // enum encoded as u8, // Index 172
    pub animation_mode: AnimationMode, // enum encoded as u8, // Index 173
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
    println!("Preset filename: {}", preset_filename);
    let encoded = encode_preset_csv(preset);

    println!("Saving preset {} to {}", preset_index, preset_filename);
    println!("Preset data: {}", encoded);

    std::fs::write(preset_filename, encoded)?;
    Ok(())
}

pub fn load_preset(index: u8) -> std::io::Result<Preset> {
    let preset_filename = format!("{}/preset_{}.bin", PRESET_PATH, index);
    let data = std::fs::read(preset_filename)?;
    let preset: Preset = decode_preset_csv(std::str::from_utf8(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("UTF-8 decoding error: {}", e)))?
    )?;
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

use flate2::write::ZlibEncoder;
use flate2::Compression;
use flate2::read::ZlibDecoder;

#[derive(Debug)] enum PresetCsvError { InvalidFormat(String), ParseError(String), IoError(std::io::Error), }

impl std::fmt::Display for PresetCsvError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { PresetCsvError::InvalidFormat(s) => write!(f, "Invalid preset CSV format: {}", s), PresetCsvError::ParseError(s) => write!(f, "Preset parse error: {}", s), PresetCsvError::IoError(e) => write!(f, "Preset IO error: {}", e), } } }

impl std::error::Error for PresetCsvError {}

impl From<PresetCsvError> for std::io::Error { fn from(err: PresetCsvError) -> Self { std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()) } }

// Helper for RGB to Hex conversion
fn rgb_to_hex(rgb: &[u8; 3]) -> String { format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]) }

// Helper for Hex to RGB conversion
fn hex_to_rgb(hex: &str) -> Result<[u8; 3], PresetCsvError> {
    // Remove the # if present
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return Err(PresetCsvError::ParseError(format!("Invalid hex color length: {}", hex))); }
    let r = u8::from_str_radix(&hex[0..2], 16) .map_err(|e| PresetCsvError::ParseError(format!("Invalid hex R value '{}': {}", &hex[0..2], e)))?;
    let g = u8::from_str_radix(&hex[2..4], 16) .map_err(|e| PresetCsvError::ParseError(format!("Invalid hex G value '{}': {}", &hex[2..4], e)))?;
    let b = u8::from_str_radix(&hex[4..6], 16) .map_err(|e| PresetCsvError::ParseError(format!("Invalid hex B value '{}': {}", &hex[4..6], e)))?;
    Ok([r, g, b])
}

// Helper for converting name byte array to String
fn name_bytes_to_string(name_bytes: &[u8; 16]) -> String {
    let len = name_bytes.iter().position(|&b| b == 0).unwrap_or(name_bytes.len());
    String::from_utf8_lossy(&name_bytes[..len]).to_string()
}

// Helper for converting String to name byte array
fn string_to_name_bytes(s: &str) -> [u8; 16] {
    let mut name_arr = [0u8; 16]; let bytes = s.as_bytes();
    let len = std::cmp::min(bytes.len(), 16); name_arr[..len].copy_from_slice(&bytes[..len]); name_arr
}

fn encode_preset_csv(preset: &Preset) -> String {
    let name_str = name_bytes_to_string(&preset.name).replace(',', " ");
    let frequencies_str = preset.frequencies.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("|");
    let gains_str = preset.gains.iter().map(|g| g.to_string()).collect::<Vec<String>>().join("|");

    format!(
        "{},{},{},{},{},{},{},{},{},[{}],[{}],{},{},{},{}",
        preset.index,
        name_str,
        preset.smooth_size,
        preset.gain,
        preset.fps,
        rgb_to_hex(&preset.color1),
        rgb_to_hex(&preset.color2),
        rgb_to_hex(&preset.color3),
        preset.fft_size,
        frequencies_str,
        gains_str,
        preset.skew,
        preset.brightness,
        preset.display_mode.clone() as u8, // Assuming DisplayMode can be cast to u8
        preset.animation_mode.clone() as u8 // Assuming AnimationMode can be cast to u8
    )
}

fn decode_preset_csv(csv: &str) -> Result<Preset, PresetCsvError> {
    let parts: Vec<&str> = csv.split(',').collect();

    if parts.len() != 15 {
        println!("CSV parts: {:?}", parts);
        return Err(PresetCsvError::InvalidFormat(format!("Expected 15 CSV parts, got {}", parts.len())));
    }

    let index = parts[0].parse::<u8>().map_err(|e| PresetCsvError::ParseError(format!("Index: {}", e)))?;
    let name_str = parts[1].trim();
    let name = string_to_name_bytes(name_str);
    println!("Decoded preset name: {}", name_str);

    let smooth_size = parts[2].parse::<u16>().map_err(|e| PresetCsvError::ParseError(format!("Smooth Size: {}", e)))?;
    let gain = parts[3].parse::<f32>().map_err(|e| PresetCsvError::ParseError(format!("Gain: {}", e)))?;
    let fps = parts[4].parse::<u16>().map_err(|e| PresetCsvError::ParseError(format!("FPS: {}", e)))?;
    println!("Decoded smooth size: {}, gain: {:.3}, fps: {}", smooth_size, gain, fps);

    let color1 = hex_to_rgb(parts[5])?;
    let color2 = hex_to_rgb(parts[6])?;
    let color3 = hex_to_rgb(parts[7])?;
    println!("Decoded colors: {:?}, {:?}, {:?}", color1, color2, color3);

    let fft_size = parts[8].parse::<u16>().map_err(|e| PresetCsvError::ParseError(format!("FFT Size: {}", e)))?;
    println!("Decoded FFT size: {}", fft_size);

    let parse_f32_array = |s: &str, context: &str| -> Result<[f32; 22], PresetCsvError> {
        let content = s.strip_prefix('[').unwrap_or(s).strip_suffix(']').unwrap_or(s);
        let vec_f32: Vec<f32> = content.split('|')
            .map(|val_str| val_str.parse::<f32>().map_err(|e| PresetCsvError::ParseError(format!("Failed to parse f32 for {} value '{}': {}", context, val_str, e))))
            .collect::<Result<Vec<f32>, _>>()?;
        vec_f32.try_into().map_err(|_| PresetCsvError::ParseError(format!("{} array must contain exactly 22 values, got {}", context, parts.len())))
    };

    let frequencies = parse_f32_array(parts[9], "Frequencies")?;
    let gains = parse_f32_array(parts[10], "Gains")?;

    let skew = parts[11].parse::<f32>().map_err(|e| PresetCsvError::ParseError(format!("Skew: {}", e)))?;
    let brightness = parts[12].parse::<f32>().map_err(|e| PresetCsvError::ParseError(format!("Brightness: {}", e)))?;
    println!("Decoded skew: {:.3}, brightness: {:.3}", skew, brightness);

    let display_mode_val = parts[13].parse::<u8>().map_err(|e| PresetCsvError::ParseError(format!("Display Mode: {}", e)))?;
    let display_mode = DisplayMode::from_u8(display_mode_val) // Assuming DisplayMode::from_u8(u8) -> Option<DisplayMode>
        .ok_or_else(|| PresetCsvError::ParseError(format!("Invalid display mode code: {}", display_mode_val)))?;

    let animation_mode_val = parts[14].parse::<u8>().map_err(|e| PresetCsvError::ParseError(format!("Animation Mode: {}", e)))?;
    let animation_mode = AnimationMode::from_u8(animation_mode_val) // Assuming AnimationMode::from_u8(u8) -> Option<AnimationMode>
        .ok_or_else(|| PresetCsvError::ParseError(format!("Invalid animation mode code: {}", animation_mode_val)))?;

    Ok(Preset {
        index,
        name,
        smooth_size,
        gain,
        fps,
        color1,
        color2,
        color3,
        fft_size,
        frequencies,
        gains,
        skew,
        brightness,
        display_mode,
        animation_mode,
    })
}

pub fn encode_preset(preset: &Preset) -> std::io::Result<Vec<u8>> {
    let csv_data = encode_preset_csv(preset);
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(csv_data.as_bytes())?; encoder.finish().map_err(Into::into)
}

pub fn decode_preset(data: &[u8]) -> std::io::Result<Preset> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_bytes = Vec::new();
    println!("Decoding preset data of length: {}", data.len());
    decoder.read_to_end(&mut decompressed_bytes)?;
    println!("Decompressed preset data length: {}", decompressed_bytes.len());
    let content_string = String::from_utf8(decompressed_bytes.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("UTF-8 decoding error: {}", e)))?;
    println!("Decoded preset content: {}", content_string);
    let csv_data = String::from_utf8(decompressed_bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("UTF-8 decoding error: {}", e)))?;
    decode_preset_csv(&csv_data).map_err(Into::into)
}


