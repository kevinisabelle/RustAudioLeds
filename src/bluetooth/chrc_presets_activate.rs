use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::constants::GATT_PRESET_ACTIVATE_UUID;
use crate::{extend_chrc_props, object_path};
use crate::presets::load_preset;
use crate::settings::Settings;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error as ZbusError};
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct PresetActivateChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl PresetActivateChrc {
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            Self {
                base: BaseGattCharacteristic::new(
                    path,
                    GATT_PRESET_ACTIVATE_UUID.to_string(),
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

pub(crate) struct PresetActivateChrcInterface(pub Arc<Mutex<PresetActivateChrc>>);

#[gatt_characteristic()]
impl PresetActivateChrcInterface {
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        if value.len() != 1 {
            return Err(zbus::fdo::Error::InvalidArgs("Requires 1 byte".into()));
        }
        let id = value[0];
        let preset = load_preset(id)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Load preset failed: {}", e)))?;

        let preset_name_vec = preset.name.clone().to_vec();
        let preset_name = String::from_utf8(preset_name_vec)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Invalid preset name: {}", e)))?;
        let locked = self.0.lock().unwrap();
        let mut settings = locked.settings.lock().unwrap();

        preset.apply_to_settings(&mut settings);

        println!("Activated preset ID: {}, Name: {}", id, preset_name);
        Ok(())
    }
}

pub async fn get_preset_activate_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<PresetActivateChrc>>, ZbusError> {
    let chrc = Arc::new(Mutex::new(PresetActivateChrc::new(
        format!("{}/preset_activate_ch", service_path),
        service_path,
        settings,
    )));
    let path = chrc.lock().unwrap().object_path().clone();
    let interface = PresetActivateChrcInterface(chrc.clone());
    register_object_with_path(connection, path, interface).await?;
    Ok(chrc)
}