# AudioLEDs Home Assistant Integration

This custom integration allows you to control and manage your AudioLEDs device from within Home Assistant.

## Features

- **Device Control**: Turn the device on/off, adjust brightness, and change the color.
- **Advanced Settings**: Control gain, smoothing, skew, display mode, animation mode, and FFT size.
- **Preset Management**: Save, activate, and delete presets directly from your dashboard.

## Prerequisites

1.  **AudioLEDs Device**: A running AudioLEDs device on your network.
2.  **HACS (Home Assistant Community Store)**: Required for installing the custom cards for the Lovelace UI. If you don't have it, see the [HACS installation guide](https://hacs.xyz/docs/setup/download).
3.  **Custom Cards**:
    -   [auto-entities](https://github.com/thomasloven/lovelace-auto-entities)
    -   [button-card](https://github.com/custom-cards/button-card)
    
    Install these from HACS by going to **HACS > Frontend** and searching for them.

## 1. Installation

1.  **Copy the Integration Files**:
    -   Copy the `custom_components/audioleds` directory from this repository.
    -   Paste it into the `custom_components` directory of your Home Assistant configuration folder. If you don't have a `custom_components` directory, create one.

    Your directory structure should look like this:
    ```
    <config>/
    ├── custom_components/
    │   └── audioleds/
    │       ├── __init__.py
    │       ├── config_flow.py
    │       └── ... (all other integration files)
    └── ... (your other configuration files)
    ```

2.  **Restart Home Assistant**:
    -   Restart your Home Assistant instance to allow it to load the new integration. You can do this from **Settings > System > Restart**.

## 2. Configuration

1.  **Add the Integration**:
    -   Go to **Settings > Devices & Services**.
    -   Click the **+ ADD INTEGRATION** button in the bottom right.
    -   Search for "AudioLEDs" and select it.

2.  **Enter Device Details**:
    -   You will be prompted to enter the **Host** (IP address) and **Port** of your AudioLEDs device.
    -   Click **Submit**. The integration will connect to the device and, if successful, will add it to Home Assistant.

## 3. Lovelace UI Setup

This setup provides a complete dashboard to control your device and manage presets.

### Step 1: Create Helper Entities

For the "Save Preset" functionality, you need to create two helper entities.

1.  Go to **Settings > Devices & Services > Helpers**.
2.  **Create a Number Helper**:
    -   Click **+ CREATE HELPER** and choose **Number**.
    -   **Name**: `AudioLEDs Preset Index`
    -   **Minimum value**: `0`
    -   **Maximum value**: `19`
    -   **Step size**: `1`
    -   Click **CREATE**. This will create an entity like `input_number.audioleds_preset_index`.
3.  **Create a Text Helper**:
    -   Click **+ CREATE HELPER** and choose **Text**.
    -   **Name**: `AudioLEDs Preset Name`
    -   Click **CREATE**. This will create an entity like `input_text.audioleds_preset_name`.

### Step 2: Add the Lovelace Dashboard

1.  **Find Your Entity IDs**:
    -   Before you can use the dashboard configuration, you need to find the actual entity IDs for your AudioLEDs device.
    -   Go to **Settings > Devices & Services > Entities** and search for "audioleds". Note down the entity IDs (e.g., `light.audioleds_192_168_1_123`, `sensor.audioleds_192_168_1_123_presets`, etc.).

2.  **Create a New Dashboard View**:
    -   Open one of your dashboards and click the three dots in the top right to **Edit Dashboard**.
    -   Click the **+** button to add a new view.
    -   Click the three dots again and select **Raw configuration editor**.

3.  **Copy and Paste the UI Code**:
    -   Copy the entire code block below.
    -   Paste it into the raw editor.
    -   **IMPORTANT**: Replace all placeholder entity IDs (like `light.audioleds_your_device` and `sensor.audioleds_your_device_presets`) with the actual entity IDs you found in the previous step.

```yaml
# =================================================================================
# Main Device Control Card
# =================================================================================
type: entities
title: AudioLEDs Control
entities:
  # Replace 'light.audioleds_rasp5' with your actual light entity ID
  - entity: light.audioleds_rasp5
  # Replace the following entities with your actual sensor/number/select entity IDs
  - entity: number.audioleds_rasp5_gain
  - entity: number.audioleds_rasp5_smooth_size
  - entity: number.audioleds_rasp5_skew
  - entity: select.audioleds_rasp5_display_mode
  - entity: select.audioleds_rasp5_animation_mode
  - entity: select.audioleds_rasp5_fft_size

# =================================================================================
# Preset Management Section
# =================================================================================

# ---------------------------------------------------------------------------------
# A. Save Preset Card
# ---------------------------------------------------------------------------------
type: entities
title: Save New Preset
entities:
  # These helpers should match the ones you created
  - entity: input_number.audioleds_preset_index
    name: Preset Slot (0-19)
  - entity: input_text.audioleds_preset_name
    name: Preset Name
footer:
  type: buttons
  entities:
    - name: Save Preset
      icon: mdi:content-save
      show_state: false
      tap_action:
        action: call-service
        service: audioleds.save_preset
        target:
          # Replace with one of your actual device entities
          entity_id: light.audioleds_rasp5
        data:
          preset_index: "{{ states('input_number.audioleds_preset_index') | int }}"
          preset_name: "{{ states('input_text.audioleds_preset_name') }}"

# ---------------------------------------------------------------------------------
# B. Preset List and Actions Card
# ---------------------------------------------------------------------------------
# Requires auto-entities and button-card from HACS.
type: custom:auto-entities
card:
  type: entities
  title: Saved Presets
filter:
  template: >
    {% for preset in state_attr('sensor.audioleds_rasp5_presets', 'presets') %}
      {{
        {
          'type': 'custom:button-card',
          'entity': 'sensor.audioleds_rasp5_presets',
          'name': preset.name ~ ' (Slot ' ~ preset.index ~ ')',
          'template': 'list_item',
          'tap_action': {
            'action': 'call-service',
            'service': 'audioleds.activate_preset',
            'target': {
              'entity_id': 'sensor.audioleds_rasp5_presets'
            },
            'data': { 'preset_index': preset.index }
          },
          'hold_action': {
            'action': 'call-service',
            'service': 'audioleds.delete_preset',
            'target': {
              'entity_id': 'sensor.audioleds_rasp5_presets'
            },
            'data': { 'preset_index': preset.index }
          }
        }
      }},
    {% endfor %}
```

4.  **Save the Dashboard**:
    -   Click **SAVE** in the top right of the editor, then click the **X** to exit.

You should now have a fully functional dashboard for your AudioLEDs device!
