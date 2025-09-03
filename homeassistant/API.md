# AudioLeds HTTP API Documentation

Version: v1
Base URL: https://<host>:<port>/api/v1
Media Type: application/json

---
## Overview
This API exposes runtime visualization settings, allows partial updates, and manages presets (persisted locally as CSV-encoded, optionally compressed binary files per slot).

---
## Authentication
(No authentication at present. All endpoints are open. Add TLS certificates via cert.pem / key.pem.)

---
## Endpoints Summary
| Method | Path | Description |
| ------ | ---- | ----------- |
| GET | /info | Get full current Settings snapshot |
| POST | /command | Apply partial update to Settings (patch-style) |
| GET | /presets | List available presets (lightweight PresetInfo) |
| GET | /presets/{id} | Retrieve full Preset by index |
| POST | /presets | Create/overwrite a Preset (full body) |
| POST | /presets/save_current | Save current Settings into a preset slot (name + index) |
| POST | /presets/{id}/activate | Load preset into active Settings |
| DELETE | /presets/{id} | Delete a preset file |

Status codes used: 200 OK, 201 Created, 204 No Content, 404 Not Found, 500 Internal Server Error.

---
## Data Models
### Enum: DisplayMode
| Value | JSON String (accepted) | Meaning |
| ----- | ---------------------- | ------- |
| 0 | "Spectrum" (when serialized) | Frequency bars visualization |
| 1 | "Oscilloscope" | Waveform view |
| 2 | "ColorGradient" | Gradient / ambient style |

### Enum: AnimationMode
| Value | JSON String (accepted) | Meaning |
| ----- | ---------------------- | ------- |
| 0 | "Full" | Fill columns full height |
| 1 | "FullWithMax" | Full + marker of peak |
| 2 | "Points" | Dots only |
| 3 | "FullMiddle" | Mirror from center outward |
| 4 | "FullMiddleWithMax" | Mirrored + peak marker |
| 5 | "PointsMiddle" | Mirrored points |

### Object: Color
```
{
  "r": 0-255,
  "g": 0-255,
  "b": 0-255
}
```

### Object: Settings (GET /info, POST /command response)
```
{
  "smooth_size": number,
  "gain": number,
  "fps": number,
  "color1": Color,
  "color2": Color,
  "color3": Color,
  "fft_size": number,
  "frequencies": number[22],
  "gains": number[22],
  "skew": number,
  "brightness": number (0..1),
  "display_mode": "Spectrum"|"Oscilloscope"|"ColorGradient",
  "animation_mode": "Full"|"FullWithMax"|"Points"|"FullMiddle"|"FullMiddleWithMax"|"PointsMiddle",
  "led_buffer": byte[],      // Opaque to clients
  "cached_df": number,       // Derived: SAMPLE_RATE / fft_size
  "selected_preset": number, // UI selection index
  "active_preset": number    // Active preset index (255 = none)
}
```
Notes:
- led_buffer length: (NUM_LEDS * 3) + 1 (last byte often end marker for hardware protocol)
- frequencies & gains arrays are fixed length (22). Provide the complete array if modifying.

### Object: Command (POST /command request)
All fields optional; only supplied keys are updated.
```
{
  "smooth_size"?: number,
  "gain"?: number,
  "fps"?: number,
  "color1"?: Color,
  "color2"?: Color,
  "color3"?: Color,
  "fft_size"?: number,
  "frequencies"?: number[22],
  "gains"?: number[22],
  "skew"?: number,
  "brightness"?: number,
  "display_mode"?: "Spectrum"|"Oscilloscope"|"ColorGradient",
  "animation_mode"?: "Full"|"FullWithMax"|"Points"|"FullMiddle"|"FullMiddleWithMax"|"PointsMiddle"
}
```
Validation behaviors:
- brightness is clamped into [0.0, 1.0].
- fft_size triggers recomputation of cached_df.

### Object: Preset
Returned by GET /presets/{id} and accepted by POST /presets.
```
{
  "index": 0-255,
  "name": [u8;16],             // Null-terminated ASCII/UTF-8 subset (raw bytes array)
  "smooth_size": number,
  "gain": number,
  "fps": number,
  "color1": [r,g,b],           // Array of 3 bytes
  "color2": [r,g,b],
  "color3": [r,g,b],
  "fft_size": number,
  "frequencies": number[22],
  "gains": number[22],
  "skew": number,
  "brightness": number,
  "display_mode": 0|1|2,       // Discriminant (enum cast to u8)
  "animation_mode": 0..5       // Discriminant
}
```
Note: The preset JSON form exposes raw arrays for colors and name (consistent with internal serialization). Client helpers can convert string names <-> byte[16].

### Object: PresetInfo (lightweight)
```
{
  "index": 0-255,
  "name": "StringName"
}
```
Used by:
- GET /presets (list)
- POST /presets/save_current (request body)

---
## Endpoint Details
### GET /info
Retrieve full current Settings.
Response 200: Settings JSON.

### POST /command
Apply partial updates.
Request body: Command (subset). Missing fields remain unchanged.
Response 200: Full updated Settings.

Example request:
```
POST /api/v1/command
{
  "brightness": 0.8,
  "gain": 1.25,
  "display_mode": "Oscilloscope",
  "animation_mode": "FullMiddle"
}
```

### GET /presets
List all presets (names only).
Response 200: PresetInfo[]

### GET /presets/{id}
Fetch full preset.
- 200: Preset
- 404: Not Found / decode error message

### POST /presets
Create or overwrite a preset by supplying full Preset JSON (including name as byte[16]).
- 201 Created
- 500 Internal Server Error

### POST /presets/save_current
Persist current Settings into provided slot.
Request body: PresetInfo (with textual name). Server converts to Preset.
- 201 Created (and sets active_preset)

### POST /presets/{id}/activate
Loads preset into runtime Settings.
- 200 OK
- 404 Not Found

### DELETE /presets/{id}
Deletes preset file from storage.
- 204 No Content
- 500 Error (io)

---
## Preset File Format (On Disk)
Stored under: presets/preset_<index>.bin (UTF-8 CSV text — name commas sanitized). Despite .bin extension, content is CSV text (optionally could be compressed via encode_preset / decode_preset helpers elsewhere if used).

CSV Columns (15 total):
1. index (u8)
2. name (string; commas replaced by spaces)
3. smooth_size (u16)
4. gain (f32)
5. fps (u16)
6. color1 (#rrggbb)
7. color2 (#rrggbb)
8. color3 (#rrggbb)
9. fft_size (u16)
10. frequencies [f|f|...|f] (22 items)
11. gains [g|g|...|g] (22 items)
12. skew (f32)
13. brightness (f32)
14. display_mode (u8 discriminant)
15. animation_mode (u8 discriminant)

Example line:
```
0,Default,64,1,60,#0000ff,#ff0000,#ff00ff,1024,[41|55|65|...|13000],[1.3|1.2|...|4],1,1,0,0
```

Parsing constraints:
- frequencies & gains must contain exactly 22 values.
- Colors validated as 6-digit hex.
- display_mode & animation_mode validated against known discriminants.

---
## Command-Line Runtime Configuration
Binary accepts CLI overrides (parsed before server start):
| Flag | Alias | Value | Default | Notes |
| ---- | ----- | ----- | ------- | ----- |
| --smooth | -s | usize | DEFAULT_SMOOTH_SIZE | Smoothing window size |
| --gain | -g | f32 | GAIN | Global amplitude gain |
| --fps | -f | usize | FPS | Target frames per second |
| --color1 | -c1 | name/hex | blue | Parsed via color_from_string |
| --color2 | -c2 | name/hex | red | |
| --color3 | -c3 | name/hex | magenta | |
| --skew | -S | f32 | DEFAULT_SKEW | Visual skew factor |
| --fft_size | -F | usize | FFT_SIZE | Recomputes cached_df |
| --brightness | -b | f32 | 1.0 | Clamped (not at parse stage, but via API) |
| --display_mode | -d | spectrum|oscilloscope|color_gradient | spectrum | Case-sensitive tokens |
| --animation_mode | -a | full|full_with_max|points|full_middle|full_middle_with_max|points_middle | full | |

---
## Typical Workflow
1. Start service with desired CLI overrides.
2. Poll /info for initial Settings.
3. Apply UI adjustments via POST /command (send only changed fields).
4. Save configuration: POST /presets/save_current with {"index":n,"name":"Label"}.
5. List presets: GET /presets.
6. Activate later: POST /presets/{id}/activate.

---
## Error Handling
- Validation issues in preset parsing surface as 404 (load) or 500 (save/delete IO) with plain text body.
- POST /command silently ignores unknown fields (serde would reject structural mismatches at decode time). brightness clamped into [0,1].

---
## Example Session
```
GET  /api/v1/info -> 200 Settings
POST /api/v1/command {"gain":1.4,"brightness":0.7} -> 200 Updated Settings
POST /api/v1/presets/save_current {"index":2,"name":"Ambient"} -> 201
GET  /api/v1/presets -> 200 [ {"index":0,"name":"Default"}, {"index":2,"name":"Ambient"} ]
POST /api/v1/presets/2/activate -> 200
```

---
## Notes & Integration Tips
- Always treat led_buffer as opaque; it's internal visualization output.
- If changing fft_size, resend full frequencies/gains arrays only if you intend to alter their semantic mapping; otherwise existing arrays persist.
- Preset name truncates at 16 bytes (no multi-byte char splitting protection beyond simple truncation).
- Use /command for real-time control; avoid rewriting presets on every minor change (persist periodically instead).

---
## Future Extension Ideas (Not Implemented)
- Bearer token auth header.
- ETag / caching for /info.
- Bulk preset export/import.
- WebSocket push for real-time LED frames.

---
## License / Ownership
See project root (Readme.md / repository license). This document describes runtime API only.


