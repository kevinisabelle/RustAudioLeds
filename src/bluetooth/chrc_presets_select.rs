//! LED-Visualizer – “Preset Select Index” characteristic
//!
//! An 8-bit unsigned integer representing the currently selected preset index.
//!
//! Flags: **read** | **write-without-response**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_PRESET_SELECT_INDEX_UUID; // Example: "3E0E0012-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use crate::settings::Settings;

use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};

/// Holds the characteristic metadata.
#[derive(Debug)]
pub struct PresetSelectIndexChrc {
    pub base:  BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>
}

object_path! {
    impl PresetSelectIndexChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            let uuid  = GATT_PRESET_SELECT_INDEX_UUID.to_string();
            let flags = vec!["read".into(), "write-without-response".into()];

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                settings,
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            // Assuming settings.selected_preset is usize, cast to u8 for GATT
            let selected_index_u8 = self.settings.lock().unwrap().selected_preset as u8;
            let owned = OwnedValue::try_from(Value::from(vec![selected_index_u8])).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct PresetSelectIndexChrcInterface(pub Arc<Mutex<PresetSelectIndexChrc>>);

#[gatt_characteristic()]
impl PresetSelectIndexChrcInterface {
    /// ReadValue handler – returns the 1-byte u8 index.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked_chrc = self.0.lock().unwrap();
        let settings_guard = locked_chrc.settings.lock().unwrap();
        // Cast usize to u8 for GATT
        let selected_index_u8 = settings_guard.selected_preset as u8;
        println!("Preset Select Index read → {}", selected_index_u8);
        Ok(vec![selected_index_u8])
    }

    /// WriteValue handler – expects exactly 1 byte (u8 index).
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 1 {
            return Err(zbus::fdo::Error::InvalidArgs(
                "Preset Select Index expects exactly 1 byte (u8)".into(),
            ));
        }
        let new_selected_index_u8 = value[0];
        // Potentially add validation here if there's a max number of presets, e.g., 23
        // For now, directly set it.
        println!("Preset Select Index write ← {}", new_selected_index_u8);
        let locked_chrc = self.0.lock().unwrap();
        // Cast u8 to usize for Settings
        locked_chrc.settings.lock().unwrap().selected_preset = new_selected_index_u8 as usize;
        Ok(())
    }
}

pub async fn get_preset_select_index_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<PresetSelectIndexChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(PresetSelectIndexChrc::new(
        format!("{}/preset_select_index_ch", service_path.clone()),
        service_path.clone(),
        settings.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = PresetSelectIndexChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}