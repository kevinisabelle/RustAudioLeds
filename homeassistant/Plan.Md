# Visualizer Integration Plan (Backend)
## **Configuration Flow**: 

A setup process (`config_flow.py`) that prompts for the device's host and port and validates the connection.

## **Data Update Coordinator**: 

A central coordinator (`coordinator.py`) to poll `/api/v1/info` and `/api/v1/presets`, manage device state, and push updates.

## **Main Light Entity**: 

A primary light entity for the device.

* **On/Off Control**: Maps to setting brightness.
* **Brightness Control**: Manages the `brightness` attribute.
* **Color Control**: Manages `color1`.

## **Configuration Entities**:
* **Number Entities**: Sliders for `gain`, `smooth_size`, and `skew`.
* **Select Entities**: Dropdowns for `display_mode`, `animation_mode`, and `fft_size`.

## **Preset Management**:
* **Preset Sensor**: A sensor entity that lists the currently saved presets. Its state could be the count of saved presets, and an attribute would hold a list of `PresetInfo` objects (`{"index": ..., "name": ...}`) fetched from `GET /api/v1/presets`. This allows the UI to dynamically show which of the 20 slots are in use.
* **Services**: The integration will register three domain-level services to manage presets. This allows you to create buttons in your dashboard that call these services for any of the 20 slots.
  - `audioviz.save_preset`: Saves the current device settings to a specified slot. It will require a `preset_index` (0-19) and a `preset_name`. This service calls `POST /api/v1/presets/save_current`.
  - `audioviz.activate_preset`: Activates the settings from a specified slot. It will require a `preset_index` (0-19). This service calls `POST /api/v1/presets/{id}/activate`.
  - `audioviz.delete_preset`: Deletes the preset from a specified slot. It will require a `preset_index` (0-19). This service calls `DELETE /api/v1/presets/{id}`.

# Plan for Creating the Lovelace UI
- Create Input Helpers for Preset Management First, create helper entities that will be used to provide the preset_index and preset_name when saving a new preset.
- Go to Settings -> Devices & Services -> Helpers.
- Create a Number helper (e.g., input_number.audioviz_preset_index) with a range from 0 to 19 and step of 1.
- Create a Text helper (e.g., input_text.audioviz_preset_name) to hold the name for the new preset.
- Build the Main Device Card Use an Entities card to group the primary controls. This card will automatically display the correct UI elements (toggle, sliders, dropdowns) for each entity.
# In your Lovelace dashboard UI editor
```yaml
type: entities
title: AudioVisualizer Control
entities:
  - entity: light.audioviz_main # Main light entity
  - entity: number.audioviz_gain
  - entity: number.audioviz_smooth_size
  - entity: number.audioviz_skew
  - entity: select.audioviz_display_mode
  - entity: select.audioviz_animation_mode
  - entity: select.audioviz_fft_size
```
Build the Preset Management Section This requires a combination of cards to display the list of presets and provide controls for saving, activating, and deleting them.
A. Save Preset Card Use an Entities card to group the input helpers and a button to call the save service.
# Card for saving a new preset
```yaml
type: entities
title: Save New Preset
entities:
  - entity: input_number.audioviz_preset_index
    name: Preset Slot (0-19)
  - entity: input_text.audioviz_preset_name
    name: Preset Name
footer:
  type: buttons
  entities:
    - name: Save Preset
      icon: mdi:content-save
      show_state: false
      tap_action:
        action: call-service
        service: audioviz.save_preset
        service_data:
          preset_index: "{{ states('input_number.audioviz_preset_index') | int }}"
          preset_name: "{{ states('input_text.audioviz_preset_name') }}"
```
B. Preset List and Actions Card To dynamically display the list of saved presets with "Activate" and "Delete" buttons for each, the custom:auto-entities card combined with custom:button-card is a powerful approach. You will need to install these from HACS (Home Assistant Community Store).
This card will automatically generate a row for each preset found in the sensor's attributes.
# Card to list presets and provide actions
# Requires auto-entities and button-card from HACS
```yaml
type: custom:auto-entities
card:
  type: entities
  title: Saved Presets
filter:
  template: >
    {% for preset in state_attr('sensor.audioviz_presets', 'presets') %}
      {{
        {
          'type': 'custom:button-card',
          'entity': 'sensor.audioviz_presets',
          'name': preset.name ~ ' (Slot ' ~ preset.index ~ ')',
          'template': 'list_item',
          'tap_action': {
            'action': 'call-service',
            'service': 'audioviz.activate_preset',
            'service_data': { 'preset_index': preset.index }
          },
          'hold_action': {
            'action': 'call-service',
            'service': 'audioviz.delete_preset',
            'service_data': { 'preset_index': preset.index }
          }
        }
      }},
    {% endfor %}
```
This configuration creates a list where tapping a preset activates it, and holding it down deletes it.