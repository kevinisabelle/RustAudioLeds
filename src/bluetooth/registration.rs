use crate::bluetooth::visualizer_app::{ObjectManagerInterface, VisualizerGattApplication, VisualizerGattApplicationInterface};
use crate::bluez::utils::{find_adapter, register_object_with_path, ObjectPathTrait};
use crate::settings::Settings;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::{Connection, Error, Proxy};

