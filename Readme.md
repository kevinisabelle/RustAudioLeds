
# Bluetooth Parameters

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
