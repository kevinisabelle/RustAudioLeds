use std::fmt::{Debug, Formatter};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_slice(&self) -> [u8; 3] {
        [self.g.min(254), self.r.min(254), self.b.min(254)]
    }

    pub fn brightness(&self, factor: f32) -> Color {
        Color {
            r: (self.r as f32 * factor).min(254.0) as u8,
            g: (self.g as f32 * factor).min(254.0) as u8,
            b: (self.b as f32 * factor).min(254.0) as u8,
        }
    }
    
    pub fn mix(&self, other: &Color, factor: f32) -> Color {
        Color {
            r: ((self.r as f32 * (1.0 - factor)) + (other.r as f32 * factor)).min(254.0) as u8,
            g: ((self.g as f32 * (1.0 - factor)) + (other.g as f32 * factor)).min(254.0) as u8,
            b: ((self.b as f32 * (1.0 - factor)) + (other.b as f32 * factor)).min(254.0) as u8,
        }
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write to Hex format
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

pub const RED : Color = Color::new(254, 0, 0);
pub const GREEN : Color = Color::new(0, 254, 0);
pub const BLUE : Color = Color::new(0, 0, 254);
pub const WHITE : Color = Color::new(254, 254, 254);
pub const BLACK : Color = Color::new(0, 0, 0);
pub const YELLOW : Color = Color::new(254, 254, 0);
pub const CYAN : Color = Color::new(0, 254, 254);
pub const MAGENTA : Color = Color::new(254, 0, 254);
pub const ORANGE : Color = Color::new(254, 165, 0);
pub const PURPLE : Color = Color::new(128, 0, 128);
pub const PINK : Color = Color::new(254, 100, 100);
pub const BROWN : Color = Color::new(165, 42, 42);
pub const GRAY : Color = Color::new(128, 128, 128);
pub const LIGHT_GRAY : Color = Color::new(211, 211, 211);
pub const DARK_GRAY : Color = Color::new(169, 169, 169);
pub const LIGHT_BLUE : Color = Color::new(173, 216, 230);
pub const LIGHT_GREEN : Color = Color::new(144, 238, 144);
pub const LIGHT_YELLOW : Color = Color::new(254, 254, 224);
pub const LIGHT_CYAN : Color = Color::new(224, 254, 254);
pub const LIGHT_MAGENTA : Color = Color::new(254, 224, 254);
pub const LIGHT_ORANGE : Color = Color::new(254, 228, 181);
pub const LIGHT_PURPLE : Color = Color::new(221, 160, 221);
pub const LIGHT_PINK : Color = Color::new(254, 182, 193);
pub const LIGHT_BROWN : Color = Color::new(210, 180, 140);

pub fn color_from_string(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "red" => RED,
        "green" => GREEN,
        "blue" => BLUE,
        "white" => WHITE,
        "black" => BLACK,
        "yellow" => YELLOW,
        "cyan" => CYAN,
        "magenta" => MAGENTA,
        "orange" => ORANGE,
        "purple" => PURPLE,
        "pink" => PINK,
        "brown" => BROWN,
        "gray" => GRAY,
        "light_gray" => LIGHT_GRAY,
        "dark_gray" => DARK_GRAY,
        "light_blue" => LIGHT_BLUE,
        "light_green" => LIGHT_GREEN,
        "light_yellow" => LIGHT_YELLOW,
        "light_cyan" => LIGHT_CYAN,
        "light_magenta" => LIGHT_MAGENTA,
        "light_orange" => LIGHT_ORANGE,
        "light_purple" => LIGHT_PURPLE,
        "light_pink" => LIGHT_PINK,
        _ => WHITE, // Default to white if color not recognized
    }
}