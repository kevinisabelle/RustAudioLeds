//! LED-Visualizer – “Gains” characteristic
//!
//! A fixed-length array of 22 little-endian 32-bit IEEE-754 floats.
//! These are one-to-one per-band gains (linear).
//! Total size: 88 bytes.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_GAINS_UUID; // Example: "3E0E0009-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};
use crate::settings::Settings;

const NUM_GAINS: usize = 22;
const GAINS_BYTE_LENGTH: usize = NUM_GAINS * 4;

/// Holds the characteristic metadata plus the raw 88-byte value (22 LE f32s).
#[derive(Debug)]
pub struct GainsChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl GainsChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_GAINS_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let settings_guard = self.settings.lock().unwrap();
            let mut bytes_vec = Vec::with_capacity(GAINS_BYTE_LENGTH);
            for &float_val in settings_guard.gains.iter() {
                bytes_vec.extend_from_slice(&float_val.to_le_bytes());
            }
            let owned = OwnedValue::try_from(Value::from(bytes_vec)).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct GainsChrcInterface(pub Arc<Mutex<GainsChrc>>);

#[gatt_characteristic()]
impl GainsChrcInterface {
    /// ReadValue handler – returns the 88-byte LE float array.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked_chrc = self.0.lock().unwrap();
        let settings_guard = locked_chrc.settings.lock().unwrap();
        let mut val_bytes = Vec::with_capacity(GAINS_BYTE_LENGTH);
        for &f_val in settings_guard.gains.iter() {
            val_bytes.extend_from_slice(&f_val.to_le_bytes());
        }
        println!("Gains read ({} floats)", settings_guard.gains.len());
        Ok(val_bytes)
    }

    /// WriteValue handler – expects exactly 88 bytes (22 little-endian f32s).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != GAINS_BYTE_LENGTH {
            return Err(zbus::fdo::Error::InvalidArgs(
                format!("Gains expects exactly {} bytes ({} x f32 LE)", GAINS_BYTE_LENGTH, NUM_GAINS),
            ));
        }
        let mut new_gains = vec![0.0f32; NUM_GAINS];
        for i in 0..NUM_GAINS {
            let start = i * 4;
            let end = start + 4;
            // Ensure the slice is exactly 4 bytes before trying to convert
            let chunk: [u8; 4] = value.get(start..end)
                .ok_or_else(|| zbus::fdo::Error::InvalidArgs("Byte slice out of bounds for f32 conversion.".into()))?
                .try_into()
                .map_err(|_| zbus::fdo::Error::InvalidArgs("Failed to convert slice to [u8; 4] for f32.".into()))?;
            new_gains[i] = f32::from_le_bytes(chunk);
        }
        println!("Gains write ({} floats received)", new_gains.len());

        let locked_chrc = self.0.lock().unwrap();
        locked_chrc.settings.lock().unwrap().gains = new_gains;
        Ok(())
    }
}

pub async fn get_gains_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<GainsChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(GainsChrc::new(
        format!("{}/gains_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = GainsChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}
