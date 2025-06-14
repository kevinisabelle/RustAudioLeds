use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::constants::GATT_READ_SETTINGS_AS_PRESET_UUID; // e.g., `3E0E0018-7C7A-47B0-9FD5-1FC3044C3E63`
use crate::{extend_chrc_props, object_path};
use crate::presets::{Preset, encode_preset};
use crate::settings::Settings;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error as ZbusError};
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct SettingsAsPresetChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl SettingsAsPresetChrc {
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            Self {
                base: BaseGattCharacteristic::new(
                    path,
                    GATT_READ_SETTINGS_AS_PRESET_UUID.to_owned(),
                    vec!["read".into()],
                    service,
                    vec![],
                ),
                settings,
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let data = self.read_settings_as_preset().unwrap_or_default();
            let owned = OwnedValue::try_from(Value::from(data)).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }

        fn read_settings_as_preset(&self) -> std::io::Result<Vec<u8>> {
            let settings = self.settings.lock().unwrap();
            let name_bytes = "Current".as_bytes();
            let mut name_as_u8array = [0u8; 16];
            let len = name_bytes.len().min(16);
            name_as_u8array[..len].copy_from_slice(&name_bytes[..len]);
            let preset = Preset::from_settings(&settings.clone(), 0, name_as_u8array);
            let bytes = encode_preset(&preset);
            bytes.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}

pub(crate) struct SettingsAsPresetChrcInterface(pub Arc<Mutex<SettingsAsPresetChrc>>);

#[gatt_characteristic()]
impl SettingsAsPresetChrcInterface {
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked = self.0.lock().unwrap();
        match locked.read_settings_as_preset() {
            Ok(bytes) => {
                println!("Read Settings as Preset → {} bytes", bytes.len());
                Ok(bytes)
            }
            Err(e) => Err(zbus::fdo::Error::Failed(format!("Read failed: {}", e))),
        }
    }
}

pub async fn get_settings_as_preset_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<SettingsAsPresetChrc>>, ZbusError> {
    let chrc = Arc::new(Mutex::new(SettingsAsPresetChrc::new(
        format!("{}/settings_as_preset_ch", service_path),
        service_path,
        settings,
    )));
    let path = chrc.lock().unwrap().object_path().to_string();
    let interface = SettingsAsPresetChrcInterface(chrc.clone());
    register_object_with_path(connection, path, interface).await?;
    Ok(chrc)
}