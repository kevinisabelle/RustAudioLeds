//! LED-Visualizer – “Brightness” characteristic
//!
//! A 32-bit IEEE-754 float representing the LED brightness (0.0 to 1.0).
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_BRIGHTNESS_UUID; // Example: "3E0E000B-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};
use crate::settings::Settings;

/// Holds the characteristic metadata plus the raw 4-byte value (little-endian).
#[derive(Debug)]
pub struct BrightnessChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl BrightnessChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_BRIGHTNESS_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            // Assuming settings.brightness is a f32
            let owned = OwnedValue::try_from(Value::from(self.settings.lock().unwrap().brightness.to_le_bytes().to_vec())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct BrightnessChrcInterface(pub Arc<Mutex<BrightnessChrc>>);

#[gatt_characteristic()]
impl BrightnessChrcInterface {
    /// ReadValue handler – returns the 4-byte LE float.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        // Assuming settings.brightness is a f32
        let val_bytes = self.0.lock().unwrap().settings.lock().unwrap().brightness.to_le_bytes().to_vec();
        let val_f32 = f32::from_le_bytes([val_bytes[0], val_bytes[1], val_bytes[2], val_bytes[3]]);
        println!("Brightness read → {:.3}", val_f32);
        Ok(val_bytes)
    }

    /// WriteValue handler – expects exactly 4 bytes (little-endian f32, 0.0-1.0).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 4 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Brightness expects exactly 4 bytes (f32 LE)".into(),
            ));
        }
        let mut new_brightness = f32::from_le_bytes([value[0], value[1], value[2], value[3]]);

        // Clamp the brightness value to the 0.0-1.0 range
        if new_brightness < 0.0 {
            new_brightness = 0.0;
        } else if new_brightness > 1.0 {
            new_brightness = 1.0;
        }

        println!("Brightness write ← {:.3}", new_brightness);
        // Assuming settings.brightness is a f32
        self.0.lock().unwrap().settings.lock().unwrap().brightness = new_brightness;
        Ok(())
    }
}

pub async fn get_brightness_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<BrightnessChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(BrightnessChrc::new(
        format!("{}/brightness_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = BrightnessChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}