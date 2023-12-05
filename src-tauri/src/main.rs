// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serialport::{available_ports, Parity, SerialPort};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
pub static PORT: Mutex<Option<Box<dyn SerialPort>>> = Mutex::new(None);
#[tauri::command]
fn get_ports() -> Vec<String> {
    let ports = available_ports().unwrap();
    ports.into_iter().map(|port| port.port_name).collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
