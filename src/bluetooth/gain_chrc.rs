//! LED-Visualizer – “Gain” characteristic
//!
//! A 32-bit IEEE-754 float that scales the whole audio signal.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_GAIN_UUID;            // 3E0E0002-7C7A-47B0-9FD5-1FC3044C3E63
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
pub struct GainChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl GainChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_GAIN_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let owned = OwnedValue::try_from(Value::from(self.settings.lock().unwrap().gain.to_le_bytes().to_vec())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct GainChrcInterface(pub Arc<Mutex<GainChrc>>);

#[gatt_characteristic()]
impl GainChrcInterface {
    /// ReadValue handler – returns the 4-byte LE float.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let val = self.0.lock().unwrap().settings.lock().unwrap().gain.to_le_bytes().to_vec();
        let f   = f32::from_le_bytes([val[0], val[1], val[2], val[3]]);
        println!("Gain read → {:.3}", f);
        Ok(val)
    }

    /// WriteValue handler – expects exactly 4 bytes (little-endian f32).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 4 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Gain expects exactly 4 bytes (f32 LE)".into(),
            ));
        }
        let new_gain = f32::from_le_bytes([value[0], value[1], value[2], value[3]]);
        println!("Gain write ← {:.3}", new_gain);
        self.0.lock().unwrap().settings.lock().unwrap().gain = new_gain;
        Ok(())
    }
}

pub async fn get_gain_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<GainChrc>>, Error> {
    let gain_chrc = Arc::new(Mutex::new(GainChrc::new(
        format!("{}/gain_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let gain_object_path = gain_chrc.lock().unwrap().object_path().clone();
    let gain_chrc_interface = GainChrcInterface(gain_chrc.clone());
    register_object_with_path(
        connection,
        gain_object_path.clone(),
        gain_chrc_interface,
    ).await?;

    Ok(gain_chrc)
}

