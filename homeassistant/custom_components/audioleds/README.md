# AudioLeds Home Assistant Integration

Unofficial custom integration for an AudioLeds device exposing an HTTPS API.

## Features
- Config Flow (UI) setup (host, port, token, toggle SSL verification)
- Periodic polling of device info and presets
- Light entity exposing brightness (0-255 mapped to device 0.0-1.0)
- Services to send partial command updates and manage presets:
  - `audioleds.send_command`
  - `audioleds.create_preset`
  - `audioleds.activate_preset`
  - `audioleds.delete_preset`

## Installation
1. Copy `custom_components/audioleds` into your Home Assistant `config/custom_components` directory.
2. Restart Home Assistant.
3. Add integration via Settings -> Devices & Services -> Add Integration -> AudioLeds.
4. Enter host (without protocol), port (default 3000), bearer token, and whether to verify SSL (disable for self-signed device cert).

## Services
### audioleds.send_command
Send a partial update (POST /api/v1/command). Example:
```yaml
service: audioleds.send_command
data:
  data:
    brightness: 0.8
    fps: 60
    display_mode: Spectrum
```

### audioleds.create_preset
Create a preset (POST /api/v1/presets). Example:
```yaml
service: audioleds.create_preset
data:
  data:
    index: 0
    name: [84,101,115,116,0,0,0,0,0,0,0,0,0,0,0,0]
    brightness: 0.9
    gain: 1.1
```

### audioleds.activate_preset
```yaml
service: audioleds.activate_preset
data:
  preset_id: 0
```

### audioleds.delete_preset
```yaml
service: audioleds.delete_preset
data:
  preset_id: 0
```

## Example Automation
Turn on light with specific brightness:
```yaml
alias: AudioLeds Evening Brightness
trigger:
  - platform: time
    at: "20:00:00"
action:
  - service: light.turn_on
    target:
      entity_id: light.audioleds
    data:
      brightness: 180
``` 

## Troubleshooting
- If using a self-signed certificate, leave Verify SSL unchecked.
- Enable debug logging:
```yaml
logger:
  logs:
    custom_components.audioleds: debug
```

## Future Enhancements
- Expose additional parameters (modes, colors) via separate entities or controls
- Config options for update interval
- Sensor / select entities for presets and modes

## Disclaimer
This is an unofficial integration. Use at your own risk.

