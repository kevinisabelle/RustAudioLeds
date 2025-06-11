//! LED-Visualizer – “LED Buffer” characteristic
//!
//! The rest of the buffer from byte 500 that provides a snapshot of all pixels in physical order.
//!
//! Flags: **read**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use crate::settings::Settings; // Assuming Settings struct holds the LED buffer

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};
use crate::constants::GATT_LED_BUFFER2_UUID;

/// UUID for the LED Buffer characteristic.
/// (Typically, this would be in a central constants file like `src/constants.rs`)


/// Characteristic wrapper holding a reference to settings containing the LED buffer.
#[derive(Debug)]
pub struct LedBuffer2Chrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl LedBuffer2Chrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_LED_BUFFER2_UUID.to_string();
            let flags = vec!["read".to_string()]; // Read-only

            Self {
                settings,
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            }
        }

        /// Expose properties for `GetManagedObjects`.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            // Assuming settings.led_buffer is Vec<u8> of 792 bytes
            let buffer_value = self.settings.lock().unwrap().led_buffer.clone();
            let owned_val = OwnedValue::try_from(Value::from(buffer_value)).unwrap();
            extend_chrc_props!(&self, props, owned_val);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct LedBuffer2ChrcInterface(pub Arc<Mutex<LedBuffer2Chrc>>);

#[gatt_characteristic()]
impl LedBuffer2ChrcInterface {
    /// ReadValue – returns the current 792-byte LED buffer.
    fn read_value(
        &self,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        // Return only the bytes after byte 500
        let buffer_value = self.0.lock().unwrap().settings.lock().unwrap().led_buffer.clone();
        if buffer_value.len() < 500 {
            // Return empty list
            return Ok(vec![]);
        }

        let buffer_value = buffer_value[500..].to_vec(); // Get bytes after 500
        println!("LED Buffer read ({} bytes)", buffer_value.len());
        // println!("LED Buffer content (HEX): {:?}", buffer_value.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>());
        Ok(buffer_value)
    }

    // No WriteValue method as this characteristic is read-only.
}

pub async fn get_led_buffer2_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<LedBuffer2Chrc>>, Error> {
    let led_buffer_chrc = Arc::new(Mutex::new(LedBuffer2Chrc::new(
        format!("{}/led_buffer2_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let led_buffer_object_path = led_buffer_chrc.lock().unwrap().object_path().clone();
    let led_buffer_chrc_interface = LedBuffer2ChrcInterface(led_buffer_chrc.clone());
    register_object_with_path(
        connection,
        led_buffer_object_path.clone(),
        led_buffer_chrc_interface,
    )
        .await?;

    Ok(led_buffer_chrc)
}