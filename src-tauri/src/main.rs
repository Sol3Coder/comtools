// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use serialport::{available_ports, Parity, SerialPort};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use std::thread;
use std::time::Duration;
use tauri::Manager;

struct MessageChannel {
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
}

struct Message {
    data: String,
}
lazy_static! {
    static ref TX: Arc<Mutex<mpsc::Sender<Message>>> = {
        let (tx, _rx) = mpsc::channel();
        Arc::new(Mutex::new(tx))
    };
}

pub static PORT: Mutex<Option<Box<dyn SerialPort>>> = Mutex::new(None);
#[derive(Clone, serde::Serialize)]
struct ReadMsg {
    msg: String,
    msg_time: String,
    len: usize,
}
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
    let tx = TX.lock().unwrap();
    tx.send(Message {
        data: "test".to_string(),
    })
    .unwrap();
    if let Some(mut port) = PORT.lock().unwrap().as_deref_mut() {
        let u8_array = string_to_u8_array(add_spaces(msg).as_str());

        port.write(&u8_array).expect("write fail!");
    };
}
fn read<R: tauri::Runtime>(manager: &impl Manager<R>) {
    if let Some(mut port) = PORT.lock().unwrap().as_deref_mut() {
        let mut serial_buf: Vec<u8> = vec![0; 1024];

        let n = match port.read(serial_buf.as_mut_slice()) {
            Ok(n) => n,
            Err(_e) => 0,
        };

        if n > 0 {
            let _ = manager.emit_all(
                "readMsgEvent",
                ReadMsg {
                    msg: serial_buf[0..n]
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<String>>()
                        .join(" ")
                        .to_ascii_uppercase(),
                    msg_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    len: n,
                },
            );
        }
    };
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, open, close, send])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                // A loop that takes output from the async process and sends it
                // to the webview via a Tauri Event
                loop {
                    read(&app_handle);
                    thread::sleep(Duration::from_millis(10));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
