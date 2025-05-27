//! LED-Visualizer – “Smooth Size” characteristic
//!
//! A `u16` that tells the DSP how many frames to average for smoothing.
//
//  Flags: read | write-without-response (fast, no ACK)
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_SMOOTH_SIZE_UUID;     // 3E0E0001-…C3E63
use crate::bluez::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct SmoothSizeChrc {
    pub base:  BaseGattCharacteristic,
    /// Raw little-endian bytes (2 bytes) – stored this way so BlueZ can
    /// marshal it directly.
    pub value: Vec<u8>,
}

object_path! {
    impl SmoothSizeChrc {
        /// Create the characteristic object.
        /// `path`     – DBus object path for the characteristic
        /// `service`  – Object path of the parent VisualizerService
        pub fn new(path: String, service: String) -> Self {
            let uuid  = GATT_SMOOTH_SIZE_UUID.to_string();
            let flags = vec![
                "read".to_string(),
                "write-without-response".to_string(),
            ];

            // Default = 16-frame smoothing → 0x0010 little endian
            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: 0x0010u16.to_le_bytes().to_vec(),
            }
        }

        /// Collect DBus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let owned_val = OwnedValue::try_from(Value::from(self.value.clone())).unwrap();
            extend_chrc_props!(&self, props, owned_val);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct SmoothSizeChrcInterface(pub Arc<Mutex<SmoothSizeChrc>>);

#[gatt_characteristic()]
impl SmoothSizeChrcInterface {
    /// Handle ReadValue: return the 2-byte LE u16.
    fn read_value(
        &self,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let val = self.0.lock().unwrap().value.clone();
        println!("SmoothSize read → {}", u16::from_le_bytes([val[0], val[1]]));
        Ok(val)
    }

    /// Handle WriteValue: expect exactly 2 bytes (little-endian u16).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 2 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "SmoothSize expects exactly 2 bytes".into(),
            ));
        }
        let new_val = u16::from_le_bytes([value[0], value[1]]);
        println!("SmoothSize write ← {}", new_val);
        self.0.lock().unwrap().value = value;
        Ok(())
    }
}
