// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serialport::TTYPort;
use tauri::{Manager, State};

use std::{
    io::{BufRead, BufReader, Read},
    sync::{Arc, Mutex},
    time::Duration,
};

mod arduino;
use arduino::ArduinoStatus;

use std::thread;

#[tauri::command]
fn start(
    app_handle: tauri::AppHandle,
    state: State<Arc<Mutex<SharedState>>>,
) -> Result<String, ArduinoStatus> {
    let thread_shared_state = state.inner().clone();

    let handle = thread::spawn(move || {
        let mut shared_data = thread_shared_state.lock().unwrap();
        let mut port_inner = &mut shared_data.port;

        let reader = BufReader::new(&mut port_inner);
        for line in reader.lines() {
            if line.is_ok() {
                let data = line.unwrap_or("Reading failed".into());
                // print!("{}", data)
                // on_line(&data);
                app_handle.emit_all("serial-log", "Line");
            }
        }
    });
    Ok(String::from("ok"))
}

// init a background process on the command, and emit periodic events only to the window that used the command
// #[tauri::command]
// fn start(
//     app_handle: tauri::AppHandle,
//     state: State<'_, arduino::ArduinoState>,
// ) -> Result<String, ArduinoStatus> {
//     if let Some(port_name) = state.0.lock().unwrap().port_name() {
//         // Port name available, no need to reconnect.
//         return Ok(format!("{}", port_name));
//     }

//     let (mut port, port_name) = arduino::connect()?;

//     thread::spawn(move || {
//         // let m = Arc::<TTYPort>::into_inner(port).unwrap();
//         let mut a = &port;
//         let reader = BufReader::new(&mut a);
//         let mut line = String::new();
//         for line in reader.lines() {
//             if line.is_ok() {
//                 let data = line.unwrap_or("Reading failed".into());
//                 print!("{}", data)
//                 // on_line(&data);
//             }
//         }
//     })
//     .join();

//     let mut curr = state.0.lock().unwrap().set_port(port);

//     // let port_2 = arduino::connect_port(&port_name)?;

//     // arduino::listen_detached(port_2, move |data| {
//     //     println!("{}", data);
//     //     let _ = app_handle.emit_all("serial-log", &data);
//     // });

//     Ok(format!("{}", port_name))
// }

pub struct SharedState {
    pub port: TTYPort,
    pub handles: Vec<thread::JoinHandle<()>>,
}

impl SharedState {
    fn new(port: TTYPort) -> SharedState {
        SharedState {
            port,
            handles: Vec::new(),
        }
    }
}

fn main() {
    let (mut port, port_name) = arduino::connect().expect("Connection error");
    let mut shared_state = Arc::new(Mutex::new(SharedState::new(port)));

    tauri::Builder::default()
        .setup(|app| Ok(()))
        .manage(shared_state)
        .invoke_handler(tauri::generate_handler![
            start,
            // etc...
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// .setup(|app| {
//   // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
//   // the default value is `main`. note that it must be unique
//   let main_window = app.get_window("main").unwrap();
//   // listen to the `event-name` (emitted on the `main` window)
//   let id = main_window.listen("event-name", |event| {
//     println!("got window event-name with payload {:?}", event.payload());
//   });
//   // unlisten to the event using the `id` returned on the `listen` function
//   // an `once` API is also exposed on the `Window` struct
//   main_window.unlisten(id);
//   // emit the `event-name` event to the `main` window
//   main_window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
//   Ok(())
// })

// fn first<T>(v: &Vec<T>) -> Option<&T> {
//     v.first()
// }

// #[tauri::send]
// fn connect(state: State<'_, arduino::ArduinoState>) -> Result<String, String> {

// }

// println!("{}", format!("{:?}", arduino::find_available_ports()));

// let port = arduino::connect_port("/dev/cu.usbmodemHIDPC1");

// println!("{:?}", port);

// let mut state_guard = state.0.lock().unwrap();
// // For debug:
// let port_name: String = format!("Value");

// if !state_guard.has_port() {
//   state_guard.set_port(port_name.clone());

//   return Ok(format!("{}", port_name.clone()))
// }

// let ports = arduino::find_available_ports().unwrap();

// println!("{:?}", &ports);

// Ok(format!("test"))

// let port_info = arduino::find_port().unwrap();
// match arduino::connect_port(&port_info.port_name) {
//   Ok(port) => {
//     let mut current_state = state.0.lock().await.as_ref();

//     current_state = Some(&port);

//     println!("{}", &current_state.unwrap().name().unwrap());

//     return Ok(format!("test"))
//   },
//   Err(error) => Err(format!("{}", "No port connection made"))
// }

// #[tauri::command]
// fn greet(name: &str) -> String {
//   println!("hi");
//   format!("Hello, {}!", name)
// }
