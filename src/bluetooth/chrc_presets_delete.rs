use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::constants::GATT_PRESET_DELETE_UUID; // e.g., `3E0E0016-7C7A-47B0-9FD5-1FC3044C3E63`
use crate::{extend_chrc_props, object_path};
use crate::presets::delete_preset;
use crate::settings::Settings;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error as ZbusError};
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct PresetDeleteChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl PresetDeleteChrc {
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            Self {
                base: BaseGattCharacteristic::new(
                    path,
                    GATT_PRESET_DELETE_UUID.to_owned(),
                    vec!["write-without-response".into()],
                    service,
                    vec![],
                ),
                settings,
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let owned = OwnedValue::try_from(Value::from(Vec::<u8>::new())).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }
    }
}

pub(crate) struct PresetDeleteChrcInterface(pub Arc<Mutex<PresetDeleteChrc>>);

#[gatt_characteristic()]
impl PresetDeleteChrcInterface {
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 1 {
            return Err(zbus::fdo::Error::InvalidArgs("Requires 1 byte".into()));
        }
        let id = value[0];
        delete_preset(id)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Delete failed: {}", e)))?;
        println!("Preset Delete → id {}", id);
        Ok(())
    }
}

pub async fn get_preset_delete_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<PresetDeleteChrc>>, ZbusError> {
    let chrc = Arc::new(Mutex::new(PresetDeleteChrc::new(
        format!("{}/preset_delete_ch", service_path),
        service_path,
        settings,
    )));
    let path = chrc.lock().unwrap().object_path().to_string();
    let interface = PresetDeleteChrcInterface(chrc.clone());
    register_object_with_path(connection, path, interface).await?;
    Ok(chrc)
}