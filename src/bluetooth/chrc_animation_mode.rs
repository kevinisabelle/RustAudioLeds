//! LED-Visualizer – “Animation Mode” characteristic
//!
//! An 8-bit unsigned integer representing the animation mode:
//! 0: Full, 1: FullWithMax, 2: Points, 3: FullMiddle, 4: FullMiddleWithMax, 5: PointsMiddle.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_ANIMATION_MODE_UUID; // Example: "3E0E000D-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use crate::settings::{Settings, AnimationMode}; // Import AnimationMode

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};

/// Converts AnimationMode enum to its u8 representation.
fn animation_mode_to_u8(mode: &AnimationMode) -> u8 {
    match mode {
        AnimationMode::Full => 0,
        AnimationMode::FullWithMax => 1,
        AnimationMode::Points => 2,
        AnimationMode::FullMiddle => 3,
        AnimationMode::FullMiddleWithMax => 4,
        AnimationMode::PointsMiddle => 5,
    }
}

/// Converts u8 to AnimationMode enum.
fn u8_to_animation_mode(value: u8) -> Result<AnimationMode, zbus::fdo::Error> {
    match value {
        0 => Ok(AnimationMode::Full),
        1 => Ok(AnimationMode::FullWithMax),
        2 => Ok(AnimationMode::Points),
        3 => Ok(AnimationMode::FullMiddle),
        4 => Ok(AnimationMode::FullMiddleWithMax),
        5 => Ok(AnimationMode::PointsMiddle),
        _ => Err(zbus::fdo::Error::InvalidArgs(
            format!("Invalid value for AnimationMode: {}", value),
        )),
    }
}

/// Holds the characteristic metadata.
#[derive(Debug)]
pub struct AnimationModeChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl AnimationModeChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_ANIMATION_MODE_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let mode_u8 = animation_mode_to_u8(&self.settings.lock().unwrap().animation_mode);
            let owned = OwnedValue::try_from(Value::from(vec![mode_u8])).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct AnimationModeChrcInterface(pub Arc<Mutex<AnimationModeChrc>>);

#[gatt_characteristic()]
impl AnimationModeChrcInterface {
    /// ReadValue handler – returns the 1-byte u8.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked_chrc = self.0.lock().unwrap();
        let settings_guard = locked_chrc.settings.lock().unwrap();
        let mode_u8 = animation_mode_to_u8(&settings_guard.animation_mode);
        println!("Animation Mode read → {} ({:?})", mode_u8, settings_guard.animation_mode);
        Ok(vec![mode_u8])
    }

    /// WriteValue handler – expects exactly 1 byte (u8).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 1 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Animation Mode expects exactly 1 byte (u8)".into(),
            ));
        }
        let new_mode_u8 = value[0];
        let new_animation_mode = u8_to_animation_mode(new_mode_u8)?;

        println!("Animation Mode write ← {} ({:?})", new_mode_u8, new_animation_mode);
        let locked_chrc = self.0.lock().unwrap();
        locked_chrc.settings.lock().unwrap().animation_mode = new_animation_mode;
        Ok(())
    }
}

pub async fn get_animation_mode_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<AnimationModeChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(AnimationModeChrc::new(
        format!("{}/animation_mode_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = AnimationModeChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}