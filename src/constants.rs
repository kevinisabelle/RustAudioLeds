pub const NUM_LEDS: usize = 12 * 22;
pub const END_MARKER: u8 = 0xFF;
pub const BAUD: u32 = 500_000;
pub const PORT: &str = "/dev/ttyUSB0";        // adapt to your system
pub const FPS: u64 = 60;
pub const GAIN: f32 = 1.0; // Adjust this to change sensitivity to audio level

