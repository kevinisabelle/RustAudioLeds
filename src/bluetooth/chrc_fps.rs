//! LED-Visualizer – “FPS” characteristic
//!
//! A 16-bit unsigned integer representing the target frames per second.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_FPS_UUID; // Example: "3E0E0003-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};
use crate::settings::Settings;

/// Holds the characteristic metadata plus the raw 2-byte value (little-endian).
#[derive(Debug)]
pub struct FpsChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl FpsChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_FPS_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            // Assuming settings.fps is a u16
            let owned = OwnedValue::try_from(Value::from(self.settings.lock().unwrap().fps.to_le_bytes().to_vec())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct FpsChrcInterface(pub Arc<Mutex<FpsChrc>>);

#[gatt_characteristic()]
impl FpsChrcInterface {
    /// ReadValue handler – returns the 2-byte LE u16.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        // Assuming settings.fps is a u16
        let val_bytes = self.0.lock().unwrap().settings.lock().unwrap().fps.to_le_bytes().to_vec();
        let val_u16 = u16::from_le_bytes([val_bytes[0], val_bytes[1]]);
        println!("FPS read → {}", val_u16);
        Ok(val_bytes)
    }

    /// WriteValue handler – expects exactly 2 bytes (little-endian u16).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 2 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "FPS expects exactly 2 bytes (u16 LE)".into(),
            ));
        }
        let new_fps = u16::from_le_bytes([value[0], value[1]]);
        println!("FPS write ← {}", new_fps);
        // Assuming settings.fps is a u16
        self.0.lock().unwrap().settings.lock().unwrap().fps = new_fps as usize;
        Ok(())
    }
}

pub async fn get_fps_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<FpsChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(FpsChrc::new(
        format!("{}/fps_ch", service_path.clone()), // Changed path name
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = FpsChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}