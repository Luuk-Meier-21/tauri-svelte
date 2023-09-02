// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::{self, BufRead, Read, Write},
    str::from_utf8,
    sync::Arc,
    time::Duration,
};
use tauri::State;
use tokio::sync::Mutex;
use tokio_serial::SerialStream;

mod arduino;
use arduino::ArduinoStatus;

use std::thread;

pub struct SharedState {
    pub port: Arc<Mutex<SerialStream>>,
}

impl SharedState {
    fn new(port: Arc<Mutex<SerialStream>>) -> SharedState {
        SharedState { port }
    }

    // async fn read_port(self: Self) {
    //     loop {
    //         let mut shared_data = self.port.lock().await;
    //         let mut reader = io::BufReader::new(&mut shared_data.port);
    //         let mut line = String::new();
    //         match reader.read_line(&mut line) {
    //             Ok(bytes_read) => {
    //                 if bytes_read > 0 {
    //                     println!("Received: {}", line);
    //                 }
    //             }
    //             Err(_) => (),
    //         }
    //     }
    // }
}

#[tauri::command]
async fn listen(
    app_handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<SharedState>>>,
) -> Result<String, ArduinoStatus> {
    let thread_shared_state = state.inner().clone();

    tokio::spawn(async move {
        loop {
            let mut shared_data = thread_shared_state.lock().await;
            let mut reader = io::BufReader::new(&mut shared_data.port);
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        println!("Received: {}", line);
                    }
                }
                Err(_) => (),
            }
        }
    });
    Ok(String::from(""))
}

#[tauri::command]
async fn send(
    app_handle: tauri::AppHandle,
    state: State<'_, Arc<Mutex<SharedState>>>,
) -> Result<String, ArduinoStatus> {
    let thread_shared_state = state.inner().clone();

    println!("Try send");

    tokio::spawn(async move {
        println!("Spawn");
        let data = format!("Hello from Send");
        match thread_shared_state
            .lock()
            .await
            .port
            .write_all(data.as_bytes())
        {
            Ok(_) => {
                println!("Sent: {}", data);
                tokio::time::sleep(Duration::from_millis(200)).await; // Sleep for 1 second
            }
            Err(_) => println!("error"),
        }
    });

    Ok(String::from("hi"))
}

// #[tauri::command]
// fn port_name(state: State<Arc<Mutex<SharedState>>>) -> Result<String, ArduinoStatus> {
//     let shared_state = state.lock().unwrap();
//     match shared_state.port.name() {
//         Some(name) => Ok(name),
//         None => Err(ArduinoStatus {
//             code: 404,
//             message: "No valid arduino port found",
//         }),
//     }
// }

#[tokio::main]
async fn main() {
    // // TODO: handle cases where port is not connected on app launch.
    // let port = arduino::connect().expect("Connection error");
    // let shared_state = Arc::new(Mutex::new(SharedState::new(port)));

    let port_info = arduino::find_port().expect("NO");
    let port = SerialStream::open(
        &serialport::new(&port_info.port_name, 57600).timeout(Duration::from_millis(200)),
    )
    .expect("write");

    let shared_state = Arc::new(Mutex::new(SharedState::new(port)));

    tauri::Builder::default()
        .manage(shared_state)
        .invoke_handler(tauri::generate_handler![
            listen,
            send
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
