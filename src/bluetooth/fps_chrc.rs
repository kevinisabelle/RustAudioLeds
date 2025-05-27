//! LED-Visualizer – “FPS” characteristic
//!
//! A `u16` specifying the target **frames-per-second** for LED updates.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_FPS_UUID;             // 3E0E0003-7C7A-47B0-9FD5-1FC3044C3E63
use crate::bluez::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

/// Characteristic wrapper.
#[derive(Debug)]
pub struct FpsChrc {
    pub base:  BaseGattCharacteristic,
    pub value: Vec<u8>,                           // 2-byte LE u16
}

object_path! {
    impl FpsChrc {
        /// Create the characteristic object with a default of **60 FPS**.
        pub fn new(path: String, service: String) -> Self {
            let uuid  = GATT_FPS_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: 60u16.to_le_bytes().to_vec(),
            }
        }

        /// Properties for DBus ObjectManager.
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

pub(crate) struct FpsChrcInterface(pub Arc<Mutex<FpsChrc>>);

#[gatt_characteristic()]
impl FpsChrcInterface {
    /// ReadValue – returns 2 bytes (LE).
    fn read_value(
        &self,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let val = self.0.lock().unwrap().value.clone();
        println!("FPS read → {}", u16::from_le_bytes([val[0], val[1]]));
        Ok(val)
    }

    /// WriteValue – accepts exactly 2 bytes (LE u16).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 2 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "FPS expects exactly 2 bytes".into(),
            ));
        }
        let fps = u16::from_le_bytes([value[0], value[1]]);
        println!("FPS write ← {}", fps);
        self.0.lock().unwrap().value = value;
        Ok(())
    }
}
