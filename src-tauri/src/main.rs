// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use serialport as SerialPort;
use serialport::SerialPortType;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn greet(window: Window) {
  let ports = SerialPort::available_ports().unwrap();

  for port in ports.iter() {
    if let SerialPortType::UsbPort(a) = port.port_type.clone() {
        println!("line: {:?}", a); // Prints 1
    } else {
        println!("Nope");
    }
  }
}

// #[tauri::command]
// fn greet(name: &str) -> String {
//   println!("hi");
//   format!("Hello, {}!", name)
// }

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
      // the default value is `main`. note that it must be unique
      let main_window = app.get_window("main").unwrap();

      // listen to the `event-name` (emitted on the `main` window)
      let id = main_window.listen("event-name", |event| {
        println!("got window event-name with payload {:?}", event.payload());
      });
      // unlisten to the event using the `id` returned on the `listen` function
      // an `once` API is also exposed on the `Window` struct
      main_window.unlisten(id);

      // emit the `event-name` event to the `main` window
      main_window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
      Ok(())
    })
    // .invoke_handler(tauri::generate_handler![init])
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}
