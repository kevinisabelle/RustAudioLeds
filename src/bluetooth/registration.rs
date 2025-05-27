use crate::bluez::advertisment::Advertisement;
use crate::constants::{ADV_APPEARANCE_GAMEPAD, GATT_SERVICE_VISUALIZER_UUID};

pub fn create_advertisement(path: String) -> Advertisement {
    let adv = Advertisement::new(
        path,
        "peripheral".to_string(),
        Some(vec![GATT_SERVICE_VISUALIZER_UUID.to_string()]),
        None,
        None,
        None,
        Some("LedVisualizer".to_string()),
        true,
        None,
        Some(ADV_APPEARANCE_GAMEPAD),
    );
    adv
}