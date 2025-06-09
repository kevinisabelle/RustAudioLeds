pub const NUM_STRIPS: usize = 22;
pub const LEDS_PER_STRIP: usize = 12;
pub const NUM_LEDS: usize = NUM_STRIPS * LEDS_PER_STRIP;
pub const END_MARKER: u8 = 0xFF;
pub const BAUD: u32 = 500_000;
pub const PORT: &str = "/dev/ttyUSB0";        // adapt to your system
pub const FPS: u64 = 60;
pub const GAIN: f32 = 7.0; // Adjust this to change sensitivity to audio level
pub const FFT_SIZE: usize = 4096; // Size of FFT buffer (4096 is more accurate, but slower, 2048 is faster)
pub const SAMPLE_RATE: u32 = 44100;
pub const DEFAULT_SMOOTH_SIZE: usize = 3; // Size of the rolling average buffer
pub const DEFAULT_SKEW: f32 = 0.75; // Default skew value

// --- Bluetooth ---
pub const BASE_UUID: &str = "3E0E{:04X}-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_SERVICE_VISUALIZER_UUID: &str = "3E0E0000-7C7A-47B0-9FD5-1FC3044C3E63";
pub const AGENT_PATH: &str = "/com/kevinisabelle/ledvisualizer/agent";
pub const ADVERT_PATH: &str = "/org/bluez/ledvisualizer/advertisement0";
pub const DEVICE_PATH: &str = "/org/bluez/hci0";
pub const APP_PATH: &str = "/com/kevinisabelle/ledvisualizer/app0";
pub const BLUEZ_SERVICE: &str = "org.bluez";
pub const BLUEZ_SERVICE_PATH: &str = "/org/bluez";
pub const ADAPTER_PATH: &str = "/org/bluez/hci0"; // adjust if needed
pub const AGENT_MANAGER_IFACE: &str = "org.bluez.AgentManager1";
pub const DBUS_PROPERTIES_IFACE: &str = "org.freedesktop.DBus.Properties";
pub const ADAPTER_IFACE: &str = "org.bluez.Adapter1";
pub const LE_ADVERTISING_MANAGER_IFACE: &str = "org.bluez.LEAdvertisingManager1";
pub const GATT_DESCRIPTOR_IFACE: &str = "org.bluez.GattDescriptor1";
pub const GATT_CHARACTERISTIC_IFACE: &str = "org.bluez.GattCharacteristic1";
pub const GATT_SERVICE_IFACE: &str = "org.bluez.GattService1";
pub const GATT_APPLICATION_IFACE: &str = "org.bluez.GattApplication1";
pub const ADV_APPEARANCE_GAMEPAD: u16 = 0x0180;

pub const GATT_LED_COUNT_UUID: &str = "3E0E000E-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_LED_BUFFER_UUID: &str = "3E0E000F-7C7A-47B0-9FD5-1FC3044C3E63";

/*
| #                 | UUID (128-bit)†                          | Properties       | Value type / size | Encoding & notes                                                                   |
| ----------------- | ---------------------------------------- | ---------------- | ----------------- | ---------------------------------------------------------------------------------- |
| **Service**       | **3E0E0000-7C7A-47B0-9FD5-1FC3044C3E63** | —                | —                 | Primary service holding all LED-visualizer settings                                |
| 1 Smooth Size     | 3E0E0001-…C3E63                          | Read · Write WoR | `u16` · 2 B       | Rolling-average window length                                                      |
| 2 Gain            | 3E0E0002-…C3E63                          | Read · Write WoR | `f32` · 4 B       | Global audio gain                                                                  |
| 3 FPS             | 3E0E0003-…C3E63                          | Read · Write WoR | `u16` · 2 B       | Target frames per second                                                           |
| 4 Color 1         | 3E0E0004-…C3E63                          | Read · Write WoR | `RGB888` · 3 B    | First palette colour                                                               |
| 5 Color 2         | 3E0E0005-…C3E63                          | Read · Write WoR | `RGB888` · 3 B    | Second palette colour                                                              |
| 6 Color 3         | 3E0E0006-…C3E63                          | Read · Write WoR | `RGB888` · 3 B    | Third palette colour                                                               |
| 7 FFT Size        | 3E0E0007-…C3E63                          | Read · Write WoR | `u16` · 2 B       | FFT length (e.g. 512, 1024)                                                        |
| 8 Frequencies     | 3E0E0008-…C3E63                          | Read · Write WoR | 22×`f32` · 88 B   | **Fixed-length array** of 22 little-endian floats (Hz)                             |
| 9 Gains           | 3E0E0009-…C3E63                          | Read · Write WoR | 22×`f32` · 88 B   | One-to-one per-band gains (linear)                                                 |
| 10 Skew           | 3E0E000A-…C3E63                          | Read · Write WoR | `f32` · 4 B       | Frequency-to-LED skew factor                                                       |
| 11 Brightness     | 3E0E000B-…C3E63                          | Read · Write WoR | `f32` · 4 B       | 0.0 – 1.0 mapped to LED PWM                                                        |
| 12 Display Mode   | 3E0E000C-…C3E63                          | Read · Write WoR | `u8` · 1 B        | 0 Spectrum, 1 Oscilloscope, 2 ColorGradient                                        |
| 13 Animation Mode | 3E0E000D-…C3E63                          | Read · Write WoR | `u8` · 1 B        | 0 Full, 1 FullWithMax, 2 Points, 3 FullMiddle, 4 FullMiddleWithMax, 5 PointsMiddle |

*/

pub const GATT_SMOOTH_SIZE_UUID: &str = "3E0E0001-7C7A-47B0-9FD5-1FC3044C3E63";

pub const GATT_GAIN_UUID: &str = "3E0E0002-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_FPS_UUID: &str = "3E0E0003-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_COLOR1_UUID: &str = "3E0E0004-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_COLOR2_UUID: &str = "3E0E0005-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_COLOR3_UUID: &str = "3E0E0006-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_FFT_SIZE_UUID: &str = "3E0E0007-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_FREQUENCIES_UUID: &str = "3E0E0008-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_GAINS_UUID: &str = "3E0E0009-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_SKEW_UUID: &str = "3E0E000A-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_BRIGHTNESS_UUID: &str = "3E0E000B-7C7A-47B0-9FD5-1FC3044C3E63";
pub const GATT_DISPLAY_MODE_UUID: &str = "3E0E000C-7C7A-47B0-9FD5-1FC3044C3E63";

pub const GATT_ANIMATION_MODE_UUID: &str = "3E0E000D-7C7A-47B0-9FD5-1FC3044C3E63";
