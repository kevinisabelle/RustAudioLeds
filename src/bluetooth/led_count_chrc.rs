//! LED-Visualizer – “LED Count” characteristic
//!
//! A `u16` that reports the number of LEDs the visualizer is configured for.
//! This value is fixed to 264 for the current build.
//!
//! Flags: **read**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
// Assuming GATT_LED_COUNT_UUID would be defined in crate::constants
// For this example, we'll define it locally.
// use crate::constants::GATT_LED_COUNT_UUID;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};
use crate::constants::GATT_LED_COUNT_UUID;

/// UUID for the LED Count characteristic.
/// (Typically, this would be in a central constants file like `src/constants.rs`)


/// The fixed value for LED count.
const LED_COUNT_VALUE: u16 = 264;

/// Characteristic wrapper. Since the value is fixed and read-only,
/// it doesn't need to store the value itself or settings.
#[derive(Debug)]
pub struct LedCountChrc {
    pub base: BaseGattCharacteristic,
}

object_path! {
    impl LedCountChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String) -> Self {
            let uuid  = GATT_LED_COUNT_UUID.to_string();
            let flags = vec!["read".to_string()]; // Read-only

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            }
        }

        /// Expose properties for `GetManagedObjects`.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let owned_val = OwnedValue::try_from(Value::from(LED_COUNT_VALUE.to_le_bytes().to_vec())).unwrap();
            extend_chrc_props!(&self, props, owned_val);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct LedCountChrcInterface(pub Arc<Mutex<LedCountChrc>>);

#[gatt_characteristic()]
impl LedCountChrcInterface {
    /// ReadValue – returns the fixed 16-bit LED count value.
    fn read_value(
        &self,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        // self.0.lock().unwrap() could be used if we needed to access LedCountChrc fields
        let val_bytes = LED_COUNT_VALUE.to_le_bytes().to_vec();
        println!("LED Count read → {}", LED_COUNT_VALUE);
        Ok(val_bytes)
    }

    // No WriteValue method as this characteristic is read-only.
}

pub async fn get_led_count_chrc(
    connection: &Connection,
    service_path: String,
) -> Result<Arc<Mutex<LedCountChrc>>, Error> {
    let led_count_chrc = Arc::new(Mutex::new(LedCountChrc::new(
        format!("{}/led_count_ch", service_path.clone()),
        service_path.clone(),
    )));
    let led_count_object_path = led_count_chrc.lock().unwrap().object_path().clone();
    let led_count_chrc_interface = LedCountChrcInterface(led_count_chrc.clone());
    register_object_with_path(
        connection,
        led_count_object_path.clone(),
        led_count_chrc_interface,
    ).await?;

    Ok(led_count_chrc)
}
