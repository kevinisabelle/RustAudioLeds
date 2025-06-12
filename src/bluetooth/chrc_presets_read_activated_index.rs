use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::constants::GATT_PRESET_READ_ACTIVATED_INDEX_UUID; // e.g., "3E0E0017-7C7A-47B0-9FD5-1FC3044C3E63"
use crate::{extend_chrc_props, object_path};
use crate::settings::Settings;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error as ZbusError};
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct PresetActivatedIndexChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl PresetActivatedIndexChrc {
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            Self {
                base: BaseGattCharacteristic::new(
                    path,
                    GATT_PRESET_READ_ACTIVATED_INDEX_UUID.to_string(),
                    vec!["read".into()],
                    service,
                    vec![],
                ),
                settings,
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let idx = self.settings.lock().unwrap().active_preset as u8;
            let owned = OwnedValue::try_from(Value::from(vec![idx])).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

pub(crate) struct PresetActivatedIndexChrcInterface(pub Arc<Mutex<PresetActivatedIndexChrc>>);

#[gatt_characteristic()]
impl PresetActivatedIndexChrcInterface {
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked = self.0.lock().unwrap();
        let idx = locked.settings.lock().unwrap().active_preset as u8;
        println!("Read Activated Preset Index → {}", idx);
        Ok(vec![idx])
    }
}

pub async fn get_preset_activated_index_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<PresetActivatedIndexChrc>>, ZbusError> {
    let chrc = Arc::new(Mutex::new(PresetActivatedIndexChrc::new(
        format!("{}/preset_activated_index_ch", service_path),
        service_path,
        settings,
    )));
    let path = chrc.lock().unwrap().object_path().clone();
    let interface = PresetActivatedIndexChrcInterface(chrc.clone());
    register_object_with_path(connection, path, interface).await?;
    Ok(chrc)
}