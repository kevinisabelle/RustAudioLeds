# Led Audio Visualizer for Raspberry Pi w/ Arduino and Android App

The LED audio visualizer is a project that combines a Raspberry Pi, an Arduino, and an Android app to create a visually appealing audio visualization system. The Raspberry Pi processes audio input, the Arduino controls the LED strip, and the Android app provides a user interface for configuration and control.

## Features
- **Audio Input**: The Raspberry Pi captures audio input from a microphone or audio source using an USB sound card.
- **Audio Processing**: The audio input is processed using Fast Fourier Transform (FFT) to extract frequency data.
- **LED Control**: The Arduino controls a WS2812B LED strip, displaying visualizations based on the processed audio data connected via USB.
- **Android App**: The app allows users to configure settings such as gain, color palette, and display modes, and to save presets for different visualizations.
- **Bluetooth Connectivity**: The app communicates with the Arduino over Bluetooth Low Energy (BLE) GATT services to send configuration changes and receive real-time updates on the LED strip.

## Tech Stack
- **Rust with Cross**: The Raspberry Pi application is written in Rust, utilizing the `cross` tool for cross-compilation.
- **Arduino**: The Arduino firmware is written in C/C++ and handles the LED strip control.
- **Kotlin**: The Android app is developed in Kotlin, providing a user-friendly interface for configuration and control.

# Bluetooth GATT Service Specification

| #                      | UUID (128-bit)†                          | Properties       | Value type / size              | Encoding & notes                                                                                                             |
|------------------------|------------------------------------------|------------------|--------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| **Service**            | **3E0E0000-7C7A-47B0-9FD5-1FC3044C3E63** | —                | —                              | Primary service holding all LED-visualizer settings                                                                          |
| 1 Smooth Size          | 3E0E0001-…C3E63                          | Read · Write WoR | `u16` · 2 B                    | Rolling-average window length                                                                                                |
| 2 Gain                 | 3E0E0002-…C3E63                          | Read · Write WoR | `f32` · 4 B                    | Global audio gain                                                                                                            |
| 3 FPS                  | 3E0E0003-…C3E63                          | Read · Write WoR | `u16` · 2 B                    | Target frames per second                                                                                                     |
| 4 Color 1              | 3E0E0004-…C3E63                          | Read · Write WoR | `RGB888` · 3 B                 | First palette colour                                                                                                         |
| 5 Color 2              | 3E0E0005-…C3E63                          | Read · Write WoR | `RGB888` · 3 B                 | Second palette colour                                                                                                        |
| 6 Color 3              | 3E0E0006-…C3E63                          | Read · Write WoR | `RGB888` · 3 B                 | Third palette colour                                                                                                         |
| 7 FFT Size             | 3E0E0007-…C3E63                          | Read · Write WoR | `u16` · 2 B                    | FFT length (e.g. 512, 1024)                                                                                                  |
| 8 Frequencies          | 3E0E0008-…C3E63                          | Read · Write WoR | 22×`f32` · 88 B                | **Fixed-length array** of 22 little-endian floats (Hz)                                                                       |
| 9 Gains                | 3E0E0009-…C3E63                          | Read · Write WoR | 22×`f32` · 88 B                | One-to-one per-band gains (linear)                                                                                           |
| 10 Skew                | 3E0E000A-…C3E63                          | Read · Write WoR | `f32` · 4 B                    | Frequency-to-LED skew factor                                                                                                 |
| 11 Brightness          | 3E0E000B-…C3E63                          | Read · Write WoR | `f32` · 4 B                    | 0.0 – 1.0 mapped to LED PWM                                                                                                  |
| 12 Display Mode        | 3E0E000C-…C3E63                          | Read · Write WoR | `u8` · 1 B                     | 0 Spectrum, 1 Oscilloscope, 2 ColorGradient                                                                                  |
| 13 Animation Mode      | 3E0E000D-…C3E63                          | Read · Write WoR | `u8` · 1 B                     | 0 Full, 1 FullWithMax, 2 Points, 3 FullMiddle, 4 FullMiddleWithMax, 5 PointsMiddle                                           |
| 14 LED Count           | 3E0E000E-…C3E63                          | Read             | `u16 · 2 B`                    | Fixed to **264** (22 × 12) for the current build, but still exposed so the phone can adapt if you change strip length later. |
| 15 LED Buffer          | 3E0E000F-…C3E63                          | Read             | `500 B` (`264 × RGB888`)       | Snapshot of all pixels in physical order. **Read-only** (no Notify).                                                         |
| 16 LED Buffer (2)      | 3E0E0010-…C3E63                          | Read             | `792 - 500 B` (`264 × RGB888`) | Same as above, but for the second rest of the buffer.                                                                        |
| 14 Preset List         | 3E0E0011-…-C3E63                         | Read             | `u8 + (up to 24 × 17)`         | Returns up to 24 entries: `{id: u8, name[16]: UTF-8}`; first byte is count                                                   |
| 15 Preset Select Index | 3E0E0012-…-C3E63                         | Read · Write WoR | `u8`                           | Sets or gets the selected preset index                                                                                       |
| 16 Preset Read         | 3E0E0013-…-C3E63                         | Read             | `222 B`                        | Returns the selected preset's binary data                                                                                    |
| 17 Preset Save         | 3E0E0014-…-C3E63                         | Write WoR        | `226 B`                        | Upload a new or updated preset.                                                                                              |
| 18 Preset Activate     | 3E0E0015-…-C3E63                         | Write WoR        | `u8`                           | Activates a preset by `id` (0–23); system applies it immediately                                                             |
