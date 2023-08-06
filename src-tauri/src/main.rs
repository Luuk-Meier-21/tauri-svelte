// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State};

use std::sync::{Arc, Mutex, MutexGuard};

mod arduino;
use arduino::{Arduino, ArduinoState, ArduinoStatus};
use serialport::{SerialPort, TTYPort};

use std::io::{BufRead, BufReader, Read};

use std::thread;
use std::time::Duration;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn connect(
    app_handle: tauri::AppHandle,
    state: State<'_, arduino::ArduinoState>,
) -> Result<String, ArduinoStatus> {
    if let Some(port_name) = state.0.lock().unwrap().port_name() {
        // Port name available, no need to reconnect.
        return Ok(format!("{}", port_name));
    }

    let (port, port_name) = arduino::connect()?;
    // let current_port = state.0.lock().unwrap().set_port(port);

    let mut port = Arc::new(port);
    let p2 = port.clone();

    thread::spawn(move || {
        let mut line = String::new();
        // loop {
        //     let len = p2.lock().unwrap().read_to_string(&mut line);
        //     thread::sleep(Duration::from_millis(400));
        //     println!("{}", line);
        // }

        let port = Arc::<TTYPort>::into_inner(p2).unwrap();
        let reader = BufReader::new(port);
        for line in reader.lines() {
            if line.is_ok() {
                println!("{:?}", line.unwrap_or("Reading failed".into()));
            }
        }
    });

    return Ok(format!("{}", port_name));
}

#[tauri::command]
async fn listen(
    app_handle: tauri::AppHandle,
    state: State<'_, arduino::ArduinoState>,
) -> Result<String, ArduinoStatus> {
    // let guard = state.0.lock().unwrap();
    // let p = guard.expect_port()?;
    // let mut port = Arc::new(Mutex::new(p));
    // let p2 = port.clone();

    // thread::spawn(move || {
    //     let mut line = String::new();
    //     loop {
    //         let len = p2.lock().unwrap().read_to_string(&mut line);
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // });
    return Ok(String::from("hi"));
}

fn main() {
    tauri::Builder::default()
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
        .manage(ArduinoState(Arc::new(Mutex::new(
            Arduino { port: None.into() }.into(),
        ))))
        // .invoke_handler(tauri::generate_handler![init])
        .invoke_handler(tauri::generate_handler![
          connect,
          listen
          // etc...
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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
