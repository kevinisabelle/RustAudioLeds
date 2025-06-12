//! LED-Visualizer – Palette Color characteristics
//!
//! An RGB888 (3-byte) value representing a palette color.
//! This module defines a generic color characteristic and specific constructors
//! for Color 1, Color 2, and Color 3.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::{GATT_COLOR1_UUID, GATT_COLOR2_UUID, GATT_COLOR3_UUID};
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};
use crate::color::Color;
use crate::settings::Settings;

/// Holds the characteristic metadata, the raw 3-byte RGB value, and an ID to distinguish colors.
#[derive(Debug)]
pub struct ColorChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
    pub color_id: u8, // 1 for Color1, 2 for Color2, 3 for Color3
}

object_path! {
    impl ColorChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>, uuid: String, color_id: u8) -> Self {
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
                color_id,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let settings_guard = self.settings.lock().unwrap();
            let color_value_vec = match self.color_id {
                1 => settings_guard.color1.to_rgb_888(),
                2 => settings_guard.color2.to_rgb_888(),
                3 => settings_guard.color3.to_rgb_888(),
                _ => panic!("Invalid color_id {} in get_properties", self.color_id), // Should not happen
            };
            let owned = OwnedValue::try_from(Value::from(color_value_vec)).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct ColorChrcInterface(pub Arc<Mutex<ColorChrc>>);

#[gatt_characteristic()]
impl ColorChrcInterface {
    /// ReadValue handler – returns the 3-byte RGB value.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked_chrc = self.0.lock().unwrap();
        let settings_guard = locked_chrc.settings.lock().unwrap();
        let val_bytes = match locked_chrc.color_id {
            1 => settings_guard.color1.to_rgb_888(),
            2 => settings_guard.color2.to_rgb_888(),
            3 => settings_guard.color3.to_rgb_888(),
            _ => return Err(zbus::fdo::Error::Failed(format!("Invalid color_id {} during read", locked_chrc.color_id))),
        };
        println!("Color {} read → {:?}", locked_chrc.color_id, val_bytes);
        Ok(val_bytes)
    }

    /// WriteValue handler – expects exactly 3 bytes (RGB888).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 3 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Color characteristic expects exactly 3 bytes (RGB888)".into(),
            ));
        }
        let new_color_array: [u8; 3] = [value[0], value[1], value[2]];

        let locked_chrc = self.0.lock().unwrap();
        println!("Color {} write ← {:?}", locked_chrc.color_id, new_color_array);
        let mut settings_guard = locked_chrc.settings.lock().unwrap();
        match locked_chrc.color_id {
            1 => settings_guard.color1 = Color::from_slice(&new_color_array),
            2 => settings_guard.color2 = Color::from_slice(&new_color_array),
            3 => settings_guard.color3 = Color::from_slice(&new_color_array),
            _ => return Err(zbus::fdo::Error::Failed(format!("Invalid color_id {} during write", locked_chrc.color_id))),
        }
        Ok(())
    }
}

async fn get_generic_color_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
    color_id: u8,
    uuid: String,
    path_segment: &str,
) -> Result<Arc<Mutex<ColorChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(ColorChrc::new(
        format!("{}/{}", service_path, path_segment),
        service_path.clone(),
        settings.clone(),
        uuid,
        color_id,
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = ColorChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;
    Ok(chrc)
}

pub async fn get_color1_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<ColorChrc>>, Error> {
    get_generic_color_chrc(
        connection,
        service_path,
        settings,
        1,
        GATT_COLOR1_UUID.to_string(),
        "color1_ch",
    ).await
}

pub async fn get_color2_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<ColorChrc>>, Error> {
    get_generic_color_chrc(
        connection,
        service_path,
        settings,
        2,
        GATT_COLOR2_UUID.to_string(),
        "color2_ch",
    ).await
}

pub async fn get_color3_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<ColorChrc>>, Error> {
    get_generic_color_chrc(
        connection,
        service_path,
        settings,
        3,
        GATT_COLOR3_UUID.to_string(),
        "color3_ch",
    ).await
}