//! src/ble/visualizer_service.rs

use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::GATT_SERVICE_VISUALIZER_UUID;

use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_option_prop, extend_service_props, object_path};

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use macros::gatt_service;
use zbus::{interface, Connection};

// ---------------------------------------------------------------------------
// Characteristic forward-declarations (implement each in its own file)
// ---------------------------------------------------------------------------
use crate::bluetooth::smooth_size_chrc::SmoothSizeChrc;
use crate::bluetooth::gain_chrc::{get_gain_chrc, GainChrc, GainChrcInterface};
use crate::bluetooth::fps_chrc::FpsChrc;
// use crate::bluetooth::color1_chrc::Color1Chrc;
// use crate::bluetooth::color2_chrc::Color2Chrc;
// use crate::bluetooth::color3_chrc::Color3Chrc;
use crate::bluetooth::fft_size_chrc::FftSizeChrc;
use crate::settings::Settings;
// use crate::bluetooth::frequencies_chrc::FrequenciesChrc;
// use crate::bluetooth::gains_chrc::GainsChrc;
// use crate::bluetooth::skew_chrc::SkewChrc;
// use crate::bluetooth::brightness_chrc::BrightnessChrc;
// use crate::bluetooth::display_mode_chrc::DisplayModeChrc;
// use crate::bluetooth::animation_mode_chrc::AnimationModeChrc;

// ---------------------------------------------------------------------------
// Service object
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct VisualizerService {
    pub base: BaseGattService,

    // 13 characteristics – added lazily when constructed elsewhere
    pub smooth_size_chrc:   Option<Arc<Mutex<SmoothSizeChrc>>>,
    pub gain_chrc:          Option<Arc<Mutex<GainChrc>>>,
    pub fps_chrc:           Option<Arc<Mutex<FpsChrc>>>,
    //pub color1_chrc:        Option<Arc<Mutex<Color1Chrc>>>,
    //pub color2_chrc:        Option<Arc<Mutex<Color2Chrc>>>,
    //pub color3_chrc:        Option<Arc<Mutex<Color3Chrc>>>,
    pub fft_size_chrc:      Option<Arc<Mutex<FftSizeChrc>>>,
    //pub frequencies_chrc:   Option<Arc<Mutex<FrequenciesChrc>>>,
    //pub gains_chrc:         Option<Arc<Mutex<GainsChrc>>>,
    //pub skew_chrc:          Option<Arc<Mutex<SkewChrc>>>,
    //pub brightness_chrc:    Option<Arc<Mutex<BrightnessChrc>>>,
    //pub display_mode_chrc:  Option<Arc<Mutex<DisplayModeChrc>>>,
    //pub animation_mode_chrc:Option<Arc<Mutex<AnimationModeChrc>>>,
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
                //color1_chrc:         None,
                //color2_chrc:         None,
                //color3_chrc:         None,
                fft_size_chrc:       None,
                //frequencies_chrc:    None,
                //gains_chrc:          None,
                //skew_chrc:           None,
                //brightness_chrc:     None,
                //display_mode_chrc:   None,
                //animation_mode_chrc: None,
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
            //extend_option_prop!(&self.color1_chrc,         properties);
            //extend_option_prop!(&self.color2_chrc,         properties);
            //extend_option_prop!(&self.color3_chrc,         properties);
            extend_option_prop!(&self.fft_size_chrc,       properties);
            //extend_option_prop!(&self.frequencies_chrc,    properties);
            //extend_option_prop!(&self.gains_chrc,          properties);
            //extend_option_prop!(&self.skew_chrc,           properties);
            //extend_option_prop!(&self.brightness_chrc,     properties);
            //extend_option_prop!(&self.display_mode_chrc,   properties);
            //extend_option_prop!(&self.animation_mode_chrc, properties);

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
        "/com/kevinisabelle/ledvisualizer/visu_serv".to_string(),
    )));

    let visualizer_service_path = visualizer_service.lock().unwrap().object_path().clone();

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

    let visualizer_service_interface = VisualizerServiceInterface(visualizer_service.clone());
    let visualizer_service_object_path = visualizer_service.lock().unwrap().object_path().clone();
    register_object_with_path(
        connection,
        visualizer_service_object_path.clone(),
        visualizer_service_interface,
    ).await?;

    Ok(visualizer_service)
}