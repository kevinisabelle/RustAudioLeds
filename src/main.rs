mod constants;
mod color;
mod settings;
mod animations;
mod dsp;
mod bluetooth;
mod bluez;
mod values;

use crate::animations::animate_leds;
use crate::bluetooth::registration::create_advertisement;
use crate::bluetooth::visualizer_app::create_and_register_application;
use crate::bluez::advertisment::register_advertisement;
use crate::bluez::agent::{register_agent, Agent};
use crate::bluez::utils::register_object;
use crate::constants::*;
use crate::dsp::process_audio_data;
use crate::settings::{display_usage, get_config};
use crate::values::StateValues;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    display_usage();
    println!("Starting LED Strip Visualizer...");

    // --- D-Bus Connection ---
    let connection = Connection::system().await?;
    println!("Connection to dbus established!");

    // --- Configuration ---
    let settings = get_config();
    let settings_mutex = Arc::new(Mutex::new(settings));
    let state_values = StateValues::new(settings_mutex.clone());
    let state_values_arc_mutex = Arc::new(Mutex::new(state_values));

    println!("Current Settings: {:?}", settings_mutex.lock().unwrap());

    // --- Audio Setup ---
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no capture device found");
    println!("Using device: {}", device.name()?);
    let config: StreamConfig = device.default_input_config()?.into();
    println!("Default input config: {:?}", config);
    let settings_mutex_for_audio = settings_mutex.clone();
    let state_values_for_audio = state_values_arc_mutex.clone();

    let input_stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            process_audio_data(data, &state_values_for_audio.clone(), &settings_mutex_for_audio.lock().unwrap().clone());
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None
    )?;
    input_stream.play()?;

    // --- Bluetooth Agent Setup ---
    let agent = Arc::new(Agent::new(AGENT_PATH.to_string()));
    register_object(&connection, agent).await?;
    register_agent(&connection, AGENT_PATH, "KeyboardDisplay").await?;

    // --- Register GATT Application ---
    _ = create_and_register_application(&connection, settings_mutex.clone()).await?;
    println!("GATT Application registered!");

    println!("Creating advertisement...");
    let advert = Arc::new(create_advertisement(ADVERT_PATH.to_string()));
    register_object(&connection, advert).await?;
    register_advertisement(&connection, ADVERT_PATH.to_string()).await?;
    println!("Advertisement registered!");

    // --- Serial Setup ---
    let mut port = serialport::new(PORT, BAUD)
        .timeout(Duration::from_millis(10))
        .open()?;

    let settings_for_serial = settings_mutex.clone();
    let states_values_for_serial = state_values_arc_mutex.clone();

    // --- Render Loop ---
    loop {
        animate_leds(&states_values_for_serial, &settings_for_serial, port.as_mut());
    }
}