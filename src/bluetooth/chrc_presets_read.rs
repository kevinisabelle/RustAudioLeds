use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_PRESET_READ_UUID;
use crate::bluez::utils::{register_object_with_path, ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use crate::settings::Settings;
use crate::presets::load_preset;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Error};
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct PresetReadChrc {
    pub base: BaseGattCharacteristic,
    pub settings: Arc<Mutex<Settings>>,
}

object_path! {
    impl PresetReadChrc {
        pub fn new(path: String, service: String, settings: Arc<Mutex<Settings>>) -> Self {
            Self {
                base: BaseGattCharacteristic::new(
                    path,
                    GATT_PRESET_READ_UUID.to_string(),
                    vec!["read".into()],
                    service,
                    vec![],
                ),
                settings,
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut props = HashMap::new();
            let bytes = self.read_preset_data().unwrap_or_else(|_| vec![]);
            let owned = OwnedValue::try_from(Value::from(bytes)).unwrap();
            extend_chrc_props!(&self, props, owned);
            props
        }

        fn read_preset_data(&self) -> std::io::Result<Vec<u8>> {
            let idx = self.settings.lock().unwrap().selected_preset as u8;
            let preset = load_preset(idx)?;
            let encoded = postcard::to_stdvec(&preset).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("Encoding error: {}", e))
            })?;
            Ok(encoded)
        }
    }
}

pub(crate) struct PresetReadChrcInterface(pub Arc<Mutex<PresetReadChrc>>);

#[gatt_characteristic()]
impl PresetReadChrcInterface {
    fn read_value(
        &self,
        _opts: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let locked = self.0.lock().unwrap();
        match locked.read_preset_data() {
            Ok(bytes) => {
                println!("Preset Read → {} bytes", bytes.len());
                Ok(bytes)
            }
            Err(e) => Err(zbus::fdo::Error::Failed(format!("Preset read failed: {}", e))),
        }
    }
}

pub async fn get_preset_read_chrc(
    connection: &Connection,
    service_path: String,
    settings: Arc<Mutex<Settings>>,
) -> Result<Arc<Mutex<PresetReadChrc>>, Error> {
    let chrc = Arc::new(Mutex::new(PresetReadChrc::new(
        format!("{}/preset_read_ch", service_path),
        service_path,
        settings,
    )));
    let object_path = chrc.lock().unwrap().object_path().clone();
    let interface = PresetReadChrcInterface(chrc.clone());
    register_object_with_path(connection, object_path, interface).await?;
    Ok(chrc)
}