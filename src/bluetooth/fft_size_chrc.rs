//! LED-Visualizer – “FFT Size” characteristic
//!
//! A `u16` that sets the FFT window length (valid examples: 512, 1024, 2048).
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_FFT_SIZE_UUID;          // 3E0E0007-7C7A-47B0-9FD5-1FC3044C3E63
use crate::bluez::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};
use crate::settings::Settings;

/// Characteristic wrapper holding raw little-endian bytes (2 B).
#[derive(Debug)]
pub struct FftSizeChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl FftSizeChrc {
        /// Build the characteristic object with a **default of 1024**.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_FFT_SIZE_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];
            let fft_size = settings.lock().unwrap().fft_size;

            Self {
                settings,
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            }
        }

        /// Expose properties for `GetManagedObjects`.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let owned = OwnedValue::try_from(Value::from(self.settings.lock().unwrap().fft_size.to_le_bytes().to_vec())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct FftSizeChrcInterface(pub Arc<Mutex<FftSizeChrc>>);

#[gatt_characteristic()]
impl FftSizeChrcInterface {
    /// ReadValue – returns the current 16-bit value.
    fn read_value(
        &self,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let val = self.0.lock().unwrap().settings.lock().unwrap().fft_size.to_le_bytes().to_vec();
        println!("FFT Size read → {}", u16::from_le_bytes([val[0], val[1]]));
        Ok(val)
    }

    /// WriteValue – expects exactly 2 bytes (u16 LE).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 2 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "FFT Size expects exactly 2 bytes (u16 LE)".into(),
            ));
        }
        let new_size = u16::from_le_bytes([value[0], value[1]]) as usize;
        println!("FFT Size write ← {}", new_size);
        self.0.lock().unwrap().settings.lock().unwrap().fft_size = new_size;
        Ok(())
    }
}
