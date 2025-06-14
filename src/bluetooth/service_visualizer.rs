use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::{GATT_SERVICE_VISUALIZER_PATH, GATT_SERVICE_VISUALIZER_UUID};

use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_option_prop, extend_service_props, object_path};

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use macros::gatt_service;
use zbus::{interface, Connection};
use crate::bluetooth::chrc_animation_mode::{get_animation_mode_chrc, AnimationModeChrc};
use crate::bluetooth::chrc_brightness::{get_brightness_chrc, BrightnessChrc};
use crate::bluetooth::chrc_color::{get_color1_chrc, get_color2_chrc, get_color3_chrc, ColorChrc};
use crate::bluetooth::chrc_display_mode::{get_display_mode_chrc, DisplayModeChrc};
use crate::bluetooth::chrc_smooth_size::{get_smooth_size_chrc, SmoothSizeChrc};
use crate::bluetooth::chrc_gain::{get_gain_chrc, GainChrc};
use crate::bluetooth::chrc_fps::{get_fps_chrc, FpsChrc};
use crate::bluetooth::chrc_fft_size::{get_fft_size_chrc, FftSizeChrc};
use crate::bluetooth::chrc_frequencies::{get_frequencies_chrc, FrequenciesChrc};
use crate::bluetooth::chrc_gains::{get_gains_chrc, GainsChrc};
use crate::bluetooth::chrc_led_count::{get_led_count_chrc, LedCountChrc};
use crate::bluetooth::chrc_leds_buffer2::{get_led_buffer2_chrc, LedBuffer2Chrc};
use crate::bluetooth::chrc_leds_buffer::{get_led_buffer_chrc, LedBufferChrc};
use crate::bluetooth::chrc_presets_list::{get_preset_list_chrc, PresetListChrc};
use crate::bluetooth::chrc_presets_read::{get_preset_read_chrc, PresetReadChrc};
use crate::bluetooth::chrc_presets_select::{get_preset_select_index_chrc, PresetSelectIndexChrc};
use crate::bluetooth::chrc_presets_save::{get_preset_save_chrc, PresetSaveChrc};
use crate::bluetooth::chrc_presets_activate::{get_preset_activate_chrc, PresetActivateChrc};
use crate::bluetooth::chrc_presets_delete::{get_preset_delete_chrc, PresetDeleteChrc};
use crate::bluetooth::chrc_presets_read_activated_index::{get_preset_activated_index_chrc, PresetActivatedIndexChrc};
use crate::bluetooth::chrc_skew::{get_skew_chrc, SkewChrc};
use crate::bluetooth::chrc_read_settings_as_preset::{get_settings_as_preset_chrc, SettingsAsPresetChrc};
use crate::settings::Settings;

// ---------------------------------------------------------------------------
// Service object
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct VisualizerService {
    pub base: BaseGattService,

    // 13 characteristics – added lazily when constructed elsewhere
    pub led_count:          Option<Arc<Mutex<LedCountChrc>>>,
    pub led_buffer_chrc:    Option<Arc<Mutex<LedBufferChrc>>>,
    pub led_buffer2_chrc:   Option<Arc<Mutex<LedBuffer2Chrc>>>,
    pub smooth_size_chrc:   Option<Arc<Mutex<SmoothSizeChrc>>>,
    pub gain_chrc:          Option<Arc<Mutex<GainChrc>>>,
    pub fps_chrc:           Option<Arc<Mutex<FpsChrc>>>,
    pub color1_chrc:        Option<Arc<Mutex<ColorChrc>>>,
    pub color2_chrc:        Option<Arc<Mutex<ColorChrc>>>,
    pub color3_chrc:        Option<Arc<Mutex<ColorChrc>>>,
    pub fft_size_chrc:      Option<Arc<Mutex<FftSizeChrc>>>,
    pub frequencies_chrc:   Option<Arc<Mutex<FrequenciesChrc>>>,
    pub gains_chrc:         Option<Arc<Mutex<GainsChrc>>>,
    pub skew_chrc:          Option<Arc<Mutex<SkewChrc>>>,
    pub brightness_chrc:    Option<Arc<Mutex<BrightnessChrc>>>,
    pub display_mode_chrc:  Option<Arc<Mutex<DisplayModeChrc>>>,
    pub animation_mode_chrc:Option<Arc<Mutex<AnimationModeChrc>>>,
    pub preset_list_chrc: Option<Arc<Mutex<PresetListChrc>>>,
    pub preset_select_index_chrc: Option<Arc<Mutex<PresetSelectIndexChrc>>>,
    pub preset_read_chrc: Option<Arc<Mutex<PresetReadChrc>>>,
    pub preset_save_chrc: Option<Arc<Mutex<PresetSaveChrc>>>,
    pub preset_activate_chrc: Option<Arc<Mutex<PresetActivateChrc>>>,
    pub preset_delete_chrc: Option<Arc<Mutex<PresetDeleteChrc>>>,
    pub preset_activated_index_chrc: Option<Arc<Mutex<PresetActivatedIndexChrc>>>,
    pub preset_read_settings_as_preset_chrc: Option<Arc<Mutex<SettingsAsPresetChrc>>>,
}

object_path! {
    impl VisualizerService {
        /// Create an empty service shell (characteristics are attached later).
        pub fn new(path: String) -> Self {
            Self {
                base: BaseGattService::new(
                    path,
                    GATT_SERVICE_VISUALIZER_UUID.to_string(),
                    true,           // primary = true
                    vec![]          // characteristic paths go here
                ),
                smooth_size_chrc:    None,
                gain_chrc:           None,
                fps_chrc:            None,
                led_count:           None,
                led_buffer_chrc:     None,
                led_buffer2_chrc:    None,
                color1_chrc:         None,
                color2_chrc:         None,
                color3_chrc:         None,
                fft_size_chrc:       None,
                frequencies_chrc:    None,
                gains_chrc:          None,
                skew_chrc:           None,
                brightness_chrc:     None,
                display_mode_chrc:   None,
                animation_mode_chrc: None,
                preset_list_chrc: None,
                preset_select_index_chrc: None,
                preset_read_chrc: None,
                preset_save_chrc: None,
                preset_activate_chrc: None,
                preset_delete_chrc: None,
                preset_activated_index_chrc: None,
                preset_read_settings_as_preset_chrc: None,
            }
        }

        /// BlueZ helper: remember each characteristic’s object-path.
        pub fn add_characteristic_path(&mut self, path: String) {
            self.base.characteristics.push(path);
        }

        /// Collect all DBus properties for ObjectManager.GetManagedObjects().
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut properties = HashMap::new();

            extend_service_props!(&self, properties);

            extend_option_prop!(&self.smooth_size_chrc,    properties);
            extend_option_prop!(&self.gain_chrc,           properties);
            extend_option_prop!(&self.fps_chrc,            properties);
            extend_option_prop!(&self.led_count,           properties);
            extend_option_prop!(&self.led_buffer_chrc,     properties);
            extend_option_prop!(&self.led_buffer2_chrc,    properties);
            extend_option_prop!(&self.color1_chrc,         properties);
            extend_option_prop!(&self.color2_chrc,         properties);
            extend_option_prop!(&self.color3_chrc,         properties);
            extend_option_prop!(&self.fft_size_chrc,       properties);
            extend_option_prop!(&self.frequencies_chrc,    properties);
            extend_option_prop!(&self.gains_chrc,          properties);
            extend_option_prop!(&self.skew_chrc,           properties);
            extend_option_prop!(&self.brightness_chrc,     properties);
            extend_option_prop!(&self.display_mode_chrc,   properties);
            extend_option_prop!(&self.animation_mode_chrc, properties);
            extend_option_prop!(&self.preset_list_chrc, properties);
            extend_option_prop!(&self.preset_select_index_chrc, properties);
            extend_option_prop!(&self.preset_read_chrc, properties);
            extend_option_prop!(&self.preset_save_chrc, properties);
            extend_option_prop!(&self.preset_activate_chrc, properties);
            extend_option_prop!(&self.preset_delete_chrc, properties); 
            extend_option_prop!(&self.preset_activated_index_chrc, properties);
            extend_option_prop!(&self.preset_read_settings_as_preset_chrc, properties);

            properties
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrappers
// ---------------------------------------------------------------------------

pub(crate) struct VisualizerServiceInterface(pub Arc<Mutex<VisualizerService>>);

#[gatt_service()]
impl VisualizerServiceInterface {}


pub async fn get_visualizer_service(
    connection: &Connection,
    settings: Arc<Mutex<Settings>>
) -> zbus::Result<Arc<Mutex<VisualizerService>>> {
    let visualizer_service = Arc::new(Mutex::new(VisualizerService::new(
        GATT_SERVICE_VISUALIZER_PATH.to_string(),
    )));

    let visualizer_service_path = visualizer_service.lock().unwrap().object_path().clone();

    // Gain characteristic
    let gain_chrc = get_gain_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(gain_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().gain_chrc = Some(gain_chrc.clone());

    // Led count characteristic
    let led_count_chrc = get_led_count_chrc(
        connection,
        visualizer_service_path.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(led_count_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().led_count = Some(led_count_chrc.clone());

    // ----- Smooth size characteristic -----
    let smooth_size_chrc = get_smooth_size_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone()
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(smooth_size_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().smooth_size_chrc = Some(smooth_size_chrc.clone());

    // ------ FPS characteristic -----
    let fps_chrc = get_fps_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(fps_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().fps_chrc = Some(fps_chrc.clone());

    // ------ Color1 characteristic -----
    let color1_chrc = get_color1_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(color1_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().color1_chrc = Some(color1_chrc.clone());

    // ------ Color2 characteristic -----
    let color2_chrc = get_color2_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(color2_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().color2_chrc = Some(color2_chrc.clone());

    // ------ Color3 characteristic -----
    let color3_chrc = get_color3_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(color3_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().color3_chrc = Some(color3_chrc.clone());

    // ------ FFT size characteristic -----
    let fft_size_chrc = get_fft_size_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(fft_size_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().fft_size_chrc = Some(fft_size_chrc.clone());

    // ------ Frequencies characteristic -----
    let freq_chrc = get_frequencies_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(freq_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().frequencies_chrc = Some(freq_chrc.clone());

    // ------ Gains characteristic -----
    let gains_chrc = get_gains_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(gains_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().gains_chrc = Some(gains_chrc.clone());

    // ------ Skew characteristic -----
    let skew_chrc = get_skew_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(skew_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().skew_chrc = Some(skew_chrc.clone());

    // ------ Brightness characteristic -----
    let brightness_chrc = get_brightness_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(brightness_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().brightness_chrc = Some(brightness_chrc.clone());

    // ------ Display mode characteristic -----
    let display_mode_chrc = get_display_mode_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(display_mode_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().display_mode_chrc = Some(display_mode_chrc.clone());

    // ------ Animation mode characteristic -----
    let animation_mode_chrc = get_animation_mode_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(animation_mode_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().animation_mode_chrc = Some(animation_mode_chrc.clone());

    // ------ Led buffer characteristics ------
    let led_buffer_chrc = get_led_buffer_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(led_buffer_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().led_buffer_chrc = Some(led_buffer_chrc.clone());

    // ------ Led buffer 2 characteristic ------
    let led_buffer2_chrc = get_led_buffer2_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(led_buffer2_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().led_buffer2_chrc = Some(led_buffer2_chrc.clone());
    
    // ------ Preset List characteristic ------
    let preset_list_chrc = get_preset_list_chrc(
        connection,
        visualizer_service_path.clone()
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_list_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_list_chrc = Some(preset_list_chrc.clone());
    

    // ------ Preset Select Index characteristic ------
    let preset_select_index_chrc = get_preset_select_index_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_select_index_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_select_index_chrc = Some(preset_select_index_chrc.clone());
    

    // ------ Preset Read characteristic ------
    let preset_read_chrc = get_preset_read_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_read_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_read_chrc = Some(preset_read_chrc.clone());
    

    // ------ Preset Save characteristic ------
    let preset_save_chrc = get_preset_save_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_save_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_save_chrc = Some(preset_save_chrc.clone());
    

    // ------ Preset Activate characteristic ------
    let preset_activate_chrc = get_preset_activate_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_activate_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_activate_chrc = Some(preset_activate_chrc.clone());
    
    // ------ Preset Delete characteristic ------
    let preset_delete_chrc = get_preset_delete_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_delete_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_delete_chrc = Some(preset_delete_chrc.clone());
    
    // ------ Preset Read Activated Index characteristic ------
    let preset_read_activated_index_chrc = get_preset_activated_index_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_read_activated_index_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_activated_index_chrc = Some(preset_read_activated_index_chrc.clone());

    // ------ Read current settings as preset characteristic ------
    let preset_read_settings_as_preset_chrc = get_settings_as_preset_chrc(
        connection,
        visualizer_service_path.clone(),
        settings.clone(),
    ).await?;
    visualizer_service
        .lock()
        .unwrap()
        .add_characteristic_path(preset_read_settings_as_preset_chrc.lock().unwrap().object_path().clone());
    visualizer_service.lock().unwrap().preset_read_settings_as_preset_chrc = Some(preset_read_settings_as_preset_chrc.clone());
    
    // ------ Service registration ------
    let visualizer_service_interface = VisualizerServiceInterface(visualizer_service.clone());
    let visualizer_service_object_path = visualizer_service.lock().unwrap().object_path().clone();
    register_object_with_path(
        connection,
        visualizer_service_object_path.clone(),
        visualizer_service_interface,
    ).await?;

    Ok(visualizer_service)
}