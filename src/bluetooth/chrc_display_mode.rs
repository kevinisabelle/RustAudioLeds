//! LED-Visualizer – “Display Mode” characteristic
//!
//! An 8-bit unsigned integer representing the display mode:
//! 0: Spectrum, 1: Oscilloscope, 2: ColorGradient.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_DISPLAY_MODE_UUID; // Example: "3E0E000C-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use crate::settings::{Settings, DisplayMode}; // Import DisplayMode

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};

/// Converts DisplayMode enum to its u8 representation.
fn display_mode_to_u8(mode: &DisplayMode) -> u8 {
    match mode {
        DisplayMode::Spectrum => 0,
        DisplayMode::Oscilloscope => 1,
        DisplayMode::ColorGradient => 2,
    }
}

/// Converts u8 to DisplayMode enum.
fn u8_to_display_mode(value: u8) -> Result<DisplayMode, zbus::fdo::Error> {
    match value {
        0 => Ok(DisplayMode::Spectrum),
        1 => Ok(DisplayMode::Oscilloscope),
        2 => Ok(DisplayMode::ColorGradient),
        _ => Err(zbus::fdo::Error::InvalidArgs(
            format!("Invalid value for DisplayMode: {}", value),
        )),
    }
}

/// Holds the characteristic metadata.
#[derive(Debug)]
pub struct DisplayModeChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl DisplayModeChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_DISPLAY_MODE_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let mode_u8 = display_mode_to_u8(&self.settings.lock().unwrap().display_mode);
            let owned = OwnedValue::try_from(Value::from(vec![mode_u8])).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct DisplayModeChrcInterface(pub Arc<Mutex<DisplayModeChrc>>);

#[gatt_characteristic()]
impl DisplayModeChrcInterface {
    /// ReadValue handler – returns the 1-byte u8.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked_chrc = self.0.lock().unwrap();
        let settings_guard = locked_chrc.settings.lock().unwrap();
        let mode_u8 = display_mode_to_u8(&settings_guard.display_mode);
        println!("Display Mode read → {} ({:?})", mode_u8, settings_guard.display_mode);
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
                "Display Mode expects exactly 1 byte (u8)".into(),
            ));
        }
        let new_mode_u8 = value[0];
        let new_display_mode = u8_to_display_mode(new_mode_u8)?;

        println!("Display Mode write ← {} ({:?})", new_mode_u8, new_display_mode);
        let locked_chrc = self.0.lock().unwrap();
        locked_chrc.settings.lock().unwrap().display_mode = new_display_mode;
        Ok(())
    }
}

pub async fn get_display_mode_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<DisplayModeChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(DisplayModeChrc::new(
        format!("{}/display_mode_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = DisplayModeChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}