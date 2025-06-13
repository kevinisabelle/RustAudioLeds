use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::constants::GATT_PRESET_SAVE_UUID; // e.g., "3E0E0014-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::{extend_chrc_props, object_path};
use crate::presets::{Preset, save_preset, decode_preset, encode_preset};
use crate::settings::Settings;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error as ZbusError};
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct PresetSaveChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl PresetSaveChrc {
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            Self {
                base: BaseGattCharacteristic::new(
                    path,
                    GATT_PRESET_SAVE_UUID.to_string(),
                    vec!["write-without-response".into()],
                    service,
                    vec![],
                ),
                settings,
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            // No read, but an empty buffer is exposed for properties
            let owned = OwnedValue::try_from(Value::from(Vec::<u8>::new())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

pub(crate) struct PresetSaveChrcInterface(pub Arc<Mutex<PresetSaveChrc>>);

#[gatt_characteristic()]
impl PresetSaveChrcInterface {
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        println!("Preset Save → value: {:?}", value);
        let preset: Preset = decode_preset(&value)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Decoding failed: {}", e)))?;

        println!("Preset Save → index: {}", preset.index);
        save_preset(&preset)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Saving failed: {}", e)))?;
        println!("Preset Save → index {}", preset.index);
        Ok(())
    }
}

pub async fn get_preset_save_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<PresetSaveChrc>>, ZbusError> {
    let chrc = Arc::new(Mutex::new(PresetSaveChrc::new(
        format!("{}/preset_save_ch", service_path),
        service_path,
        settings,
    )));
    let object_path = chrc.lock().unwrap().object_path().clone();
    let interface = PresetSaveChrcInterface(chrc.clone());
    register_object_with_path(connection, object_path, interface).await?;
    Ok(chrc)
}