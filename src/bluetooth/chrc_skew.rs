//! LED-Visualizer – “Skew” characteristic
//!
//! A 32-bit IEEE-754 float representing the frequency-to-LED skew factor.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_SKEW_UUID; // Example: "3E0E000A-7C7A-47B0-9FD5-1FC3044C3E63"
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
pub struct SkewChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl SkewChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_SKEW_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            // Assuming settings.skew is a f32
            let owned = OwnedValue::try_from(Value::from(self.settings.lock().unwrap().skew.to_le_bytes().to_vec())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct SkewChrcInterface(pub Arc<Mutex<SkewChrc>>);

#[gatt_characteristic()]
impl SkewChrcInterface {
    /// ReadValue handler – returns the 4-byte LE float.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        // Assuming settings.skew is a f32
        let val_bytes = self.0.lock().unwrap().settings.lock().unwrap().skew.to_le_bytes().to_vec();
        let val_f32 = f32::from_le_bytes([val_bytes[0], val_bytes[1], val_bytes[2], val_bytes[3]]);
        println!("Skew read → {:.3}", val_f32);
        Ok(val_bytes)
    }

    /// WriteValue handler – expects exactly 4 bytes (little-endian f32).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 4 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Skew expects exactly 4 bytes (f32 LE)".into(),
            ));
        }
        let new_skew = f32::from_le_bytes([value[0], value[1], value[2], value[3]]);
        println!("Skew write ← {:.3}", new_skew);
        // Assuming settings.skew is a f32
        self.0.lock().unwrap().settings.lock().unwrap().skew = new_skew;
        Ok(())
    }
}

pub async fn get_skew_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<SkewChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(SkewChrc::new(
        format!("{}/skew_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = SkewChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}