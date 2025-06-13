//! src/ble/visualizer_gatt_application.rs
//!
//! Application-level container for the Visualizer GATT hierarchy.
//! Exposes the BlueZ-required interfaces:
//!   • org.bluez.GattApplication1
//!   • org.freedesktop.DBus.ObjectManager
//!
//! The real service & characteristic implementations live elsewhere;
//! here we just surface them to BlueZ.

use crate::extend_option_prop;
use crate::bluez::utils::{find_adapter, register_object_with_path, ObjectInterfaces, ObjectPathTrait};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{interface, Connection, Proxy};
use crate::bluetooth::service_visualizer::{get_visualizer_service, VisualizerService};
use crate::settings::Settings;

/// **Placeholder** for the concrete service you’ll implement separately.
///
/// ```text
/// /org/example/ledviz   (VisualizerGattApplication → this file)
/// └── /service0         (VisualizerService       → to-do)
///     ├── /char0
///     ├── /char1
///     └── …
/// ```


// ---------------------------------------------------------------------------
// Gatt-application object
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VisualizerGattApplication {
    /// DBus object path of the *application*, e.g. `/org/example/ledviz`
    pub path: String,

    /// Root of the visualizer-settings service (optional until instantiated)
    pub visualizer_service: Option<Arc<Mutex<VisualizerService>>>,
}

impl ObjectPathTrait for VisualizerGattApplication {
    fn object_path(&self) -> String {
        self.path.clone()
    }
}

impl VisualizerGattApplication {
    /// Build a new app object.  Call this once at startup and export it
    /// through `zbus::ConnectionBuilder::object_server()`.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            visualizer_service: None,
        }
    }

    // ---------------------------------------------------------------------
    // Convenience: gather every DBus interface + property set we own
    // (BlueZ will call this via ObjectManager.GetManagedObjects).
    // ---------------------------------------------------------------------
    pub fn get_properties(&self) -> ObjectInterfaces {
        let mut props: ObjectInterfaces = HashMap::new();

        // If the VisualizerService has been created, merge its objects.
        extend_option_prop!(&self.visualizer_service, props);

        props
    }
}

// ---------------------------------------------------------------------------
// BlueZ-facing interfaces
// ---------------------------------------------------------------------------

/// Wrapper so zbus can generate `org.bluez.GattApplication1`.
pub(crate) struct VisualizerGattApplicationInterface(pub Arc<Mutex<VisualizerGattApplication>>);

#[interface(name = "org.bluez.GattApplication1")]
impl VisualizerGattApplicationInterface {}

/// Implements `org.freedesktop.DBus.ObjectManager`.
pub(crate) struct ObjectManagerInterface(pub Arc<Mutex<VisualizerGattApplication>>);

#[interface(name = "org.freedesktop.DBus.ObjectManager")]
impl ObjectManagerInterface {
    /// BlueZ calls this to discover the full GATT hierarchy in one round-trip.
    fn get_managed_objects(&self) -> zbus::fdo::Result<ObjectInterfaces> {
        let app = self.0.lock().expect("poisoned mutex");
        Ok(app.get_properties())
    }
}

pub async fn create_and_register_application(
    connection: &Connection,
    settings: Arc<Mutex<Settings>>,
) -> zbus::Result<(Arc<Mutex<VisualizerGattApplication>>)> {
    println!("Creating GattApplication");

    let app = Arc::new(Mutex::new(VisualizerGattApplication::new("/".to_string())));
    let app_interface = VisualizerGattApplicationInterface(app.clone());
    let app_object_manager_interface = ObjectManagerInterface(app.clone());

    let visualizer_service = get_visualizer_service(connection, settings).await?;

    app.lock().unwrap().visualizer_service = Some(visualizer_service.clone());

    let app_object_path = app.lock().unwrap().object_path().clone();
    register_object_with_path(connection, app_object_path.clone(), app_interface).await?;
    register_object_with_path(
        connection,
        app_object_path.clone(),
        app_object_manager_interface,
    )
        .await?;
    register_application(connection, app_object_path.clone().as_str()).await?;

    Ok(app.clone())
}

async fn register_application(connection: &Connection, app_path: &str) -> zbus::Result<()> {
    let adapter_path = find_adapter(connection).await?;

    let gatt_manager: Proxy = Proxy::new(
        connection,
        "org.bluez",
        adapter_path,
        "org.bluez.GattManager1",
    )
        .await?;

    // Create an empty dictionary for the options.
    let options: HashMap<String, zbus::zvariant::Value> = HashMap::new();
    let app_object_path = zbus::zvariant::ObjectPath::try_from(app_path)?;

    // Call the RegisterApplication method.
    let result = gatt_manager
        .call_method("RegisterApplication", &(app_object_path, options))
        .await?;

    println!("Registered application: {:?}", result);

    Ok(())
}

