// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serialport::{available_ports, Parity, SerialPort};
use std::sync::Mutex;
use std::thread;
use tauri::Manager;

use std::time::Duration;
pub static PORT: Mutex<Option<Box<dyn SerialPort>>> = Mutex::new(None);
#[tauri::command]
fn get_ports() -> Vec<String> {
    let ports = available_ports().unwrap();
    ports.into_iter().map(|port| port.port_name).collect()
}
#[tauri::command]
fn open(name: &str) -> bool {
    let port = serialport::new(name, 9600)
        .timeout(Duration::from_millis(10))
        .open();
    match port {
        Ok(port) => {
            *PORT.lock().unwrap() = Some(port);
            true
        }
        Err(_) => false,
    }
}
#[tauri::command]
fn close() {
    if let Some(port) = PORT.lock().unwrap().take() {
        drop(port);
    };
}
fn add_spaces(input: &str) -> String {
    let chars: Vec<char> = input.replace(" ", "").chars().collect();
    let chunks: Vec<String> = chars
        .chunks(2)
        .map(|chars| chars.iter().collect::<String>())
        .collect();
    chunks.join(" ")
}
fn string_to_u8_array(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect()
}

#[tauri::command]
fn send(msg: &str) {
    if let Some(mut port) = PORT.lock().unwrap().as_deref_mut() {
        let u8_array = string_to_u8_array(add_spaces(msg).as_str());

        port.write(&u8_array).expect("write fail!");
    };
}
#[tauri::command]
fn read() -> String {
    if let Some(mut port) = PORT.lock().unwrap().as_deref_mut() {
        let mut buf = [0; 1024];
        let n = match port.read(&mut buf) {
            Ok(n) => n,
            Err(e) => {
                println!("Error reading from serial port: {}", e);
                return String::new();
            }
        };

        if n > 0 {
            let data = String::from_utf8_lossy(&buf[0..n]);
            return data.as_ref().to_string();
        }
    };
    return String::new();
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, open, close, read, send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
