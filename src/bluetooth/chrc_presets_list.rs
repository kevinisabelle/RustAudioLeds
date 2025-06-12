//! LED-Visualizer – “Preset List” characteristic
//!
//! Returns up to 24 preset entries. The first byte is the count of entries,
//! followed by the entries themselves. Each entry consists of:
//! - id: u8 (from preset.index)
//! - name: [u8; 16] (UTF-8 string, from preset.name)
//! Total size: 1 (count) + (up to 24 * (1 (id) + 16 (name))) = max 409 bytes.
//!
//! Flags: **read**
//
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_PRESET_LIST_UUID;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::presets::{self};
use crate::{extend_chrc_props, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::zvariant::{OwnedValue, Value};
use zbus::{interface, Connection, Error};

const MAX_PRESETS_TO_LIST: usize = 24;

/// Serializes the list of presets according to the GATT characteristic specification.
/// Returns a byte vector: count (u8) followed by preset data.
fn serialize_preset_list_data() -> Result<Vec<u8>, std::io::Error> {
    let presets_list = presets::list_presets()?;

    let count = std::cmp::min(presets_list.len(), MAX_PRESETS_TO_LIST) as u8;
    let mut bytes_out = Vec::with_capacity(1 + (count as usize * 17));
    bytes_out.push(count);

    for i in 0..(count as usize) {
        let preset_item = &presets_list[i];
        bytes_out.push(preset_item.index); // id
        bytes_out.extend_from_slice(&preset_item.name); // name[16]
    }
    Ok(bytes_out)
}

/// Holds the characteristic metadata.
#[derive(Debug)]
pub struct PresetListChrc {
    pub base: BaseGattCharacteristic,
    // No settings field needed if list_presets is self-contained
}

object_path! {
    impl PresetListChrc {
        /// Build the characteristic object.
        pub fn new(path: String, service: String) -> Self {
            let uuid  = GATT_PRESET_LIST_UUID.to_string();
            let flags = vec!["read".into()]; // Read-only

            Self {
                base:  BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            }
        }

        /// Expose D-Bus properties for ObjectManager.
        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let value_bytes = serialize_preset_list_data().unwrap_or_else(|e| {
                    eprintln!("Error serializing preset list for properties: {}", e);
                    vec![0u8] // Default to count 0 on error
                });
            let owned = OwnedValue::try_from(Value::from(value_bytes)).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

// ---------------------------------------------------------------------------
// zbus interface wrapper
// ---------------------------------------------------------------------------

pub(crate) struct PresetListChrcInterface(pub Arc<Mutex<PresetListChrc>>);

#[gatt_characteristic()]
impl PresetListChrcInterface {
    /// ReadValue handler – returns the serialized preset list.
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        match serialize_preset_list_data() {
            Ok(value_bytes) => {
                let count = if value_bytes.is_empty() { 0 } else { value_bytes[0] };
                println!("Preset List read ({} presets, {} bytes)", count, value_bytes.len());
                Ok(value_bytes)
            }
            Err(e) => {
                eprintln!("Error reading preset list: {}", e);
                Err(zbus::fdo::Error::Failed(format!("Failed to serialize preset list: {}", e)))
            }
        }
    }

    // No WriteValue method as this characteristic is read-only.
}

pub async fn get_preset_list_chrc(
    connection: &Connection,
    service_path: String,
) -> Result<Arc<Mutex<PresetListChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(PresetListChrc::new(
        format!("{}/preset_list_ch", service_path.clone()),
        service_path.clone(),
    )));
    let object_path_str = chrc.lock().unwrap().object_path().clone();
    let chrc_interface = PresetListChrcInterface(chrc.clone());
    register_object_with_path(
        connection,
        object_path_str.clone(),
        chrc_interface,
    ).await?;

    Ok(chrc)
}