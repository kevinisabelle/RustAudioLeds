use std::collections::HashMap;
use zbus::proxy;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use zbus::Connection;

#[proxy(default_service = "org.bluez", interface = "org.bluez.Device1")]
trait Device1 {
    fn connect(&self) -> zbus::Result<()>;
    fn disconnect(&self) -> zbus::Result<()>;
    fn connect_profile(&self, uuid: String) -> zbus::Result<()>;
    fn disconnect_profile(&self, uuid: String) -> zbus::Result<()>;
    fn pair(&self) -> zbus::Result<()>;
    fn cancel_pairing(&self) -> zbus::Result<()>;
    fn get_service_records(&self) -> zbus::Result<Vec<Vec<u8>>>;

    #[zbus(property)]
    fn address(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn address_type(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn icon(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn class(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn paired(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn bonded(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn connected(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn trusted(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn blocked(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn alias(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn adapter(&self) -> zbus::Result<OwnedObjectPath>;
    #[zbus(property)]
    fn legacy_pairing(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn modalias(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn rssi(&self) -> zbus::Result<i16>;
    #[zbus(property)]
    fn tx_power(&self) -> zbus::Result<i16>;
    #[zbus(property)]
    fn manufacturer_data(&self) -> zbus::Result<HashMap<u16, Vec<u8>>>;
    #[zbus(property)]
    fn service_data(&self) -> zbus::Result<HashMap<String, Vec<u8>>>;
    #[zbus(property)]
    fn services_resolved(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn advertising_flags(&self) -> zbus::Result<Vec<u8>>;
    #[zbus(property)]
    fn advertising_data(&self) -> zbus::Result<HashMap<u8, Vec<u8>>>;
    #[zbus(property)]
    fn sets(&self) -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;
    #[zbus(property)]
    fn preferred_bearer(&self) -> zbus::Result<String>;
}

#[proxy(
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0",
    interface = "org.bluez.Adapter1"
)]
trait Adapter1 {
    fn get_device(&self, device_path: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    #[zbus(property)]
    fn address(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn powered(&self) -> zbus::Result<bool>;
    // fn list_devices(&self) -> zbus::Result<Vec<zbus::zvariant::ObjectPath>>;
}

#[proxy(
    default_service = "org.bluez",
    default_path = "/",
    interface = "org.freedesktop.DBus.ObjectManager"
)]
trait ObjectManager {
    fn get_managed_objects(
        &self,
    ) -> zbus::Result<HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>>;
}