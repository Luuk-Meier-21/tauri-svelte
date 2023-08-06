use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use std::thread;
use std::time::Duration;

use serialport::{SerialPort, SerialPortInfo, SerialPortType, TTYPort};

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArduinoStatus {
    pub code: u32,
    pub message: &'static str,
}

pub struct Arduino {
    pub port: Option<TTYPort>,
}

pub struct Test {}

pub struct ArduinoState(pub Arc<Mutex<Arduino>>);

impl Arduino {
    // pub fn expect_port(&mut self) -> Result<&TTYPort, ArduinoStatus> {
    //     match self.port {
    //         Some(port) => Ok(port),
    //         None => Err(ArduinoStatus {
    //             code: 404,
    //             message: "No open port to listen for.",
    //         }),
    //     }
    // }

    pub fn expect_port(&self) -> Result<&TTYPort, ArduinoStatus> {
        match &self.port {
            Some(port) => Ok(port),
            None => Err(ArduinoStatus {
                code: 404,
                message: "No open port to listen for.",
            }),
        }
    }

    pub fn has_port(&mut self) -> bool {
        self.port.is_some()
    }

    pub fn set_port(&mut self, value: TTYPort) -> &TTYPort {
        self.port = Some(value);
        return self.port.as_ref().unwrap();
    }

    pub fn port_name(&mut self) -> Option<String> {
        return match self.port.as_ref() {
            Some(port) => port.name(),
            None => None,
        };
    }
}
// pub struct ArduinoState(pub Mutex<Option<TTYPort>>);

pub fn find_available_ports() -> Result<Vec<SerialPortInfo>, ArduinoStatus> {
    match serialport::available_ports() {
        Ok(ports) => Ok(ports),
        Err(err) => Err(ArduinoStatus {
            code: 404,
            message: "No ports found",
        }),
    }
}

fn find_arduino_port(ports: Vec<SerialPortInfo>) -> Result<SerialPortInfo, ArduinoStatus> {
    for port in ports.iter() {
        if let SerialPortType::UsbPort(usb_port) = &port.port_type {
            return Ok(port.clone());
        }
    }
    Err(ArduinoStatus {
        code: 404,
        message: "No valid arduino port found",
    })
}

pub fn find_port() -> Result<SerialPortInfo, ArduinoStatus> {
    let available_ports = find_available_ports()?;
    let target_port = find_arduino_port(available_ports.clone())?;

    Ok(target_port)
}

pub fn connect_port(port_name: &str) -> Result<TTYPort, ArduinoStatus> {
    match TTYPort::open(&serialport::new(port_name, 57600)) {
        Ok(port) => Ok(port),
        Err(_err) => Err(ArduinoStatus {
            code: 400,
            message: "Unable to open port",
        }),
    }

    // let output = "This is a test. This is only a test.".as_bytes();
    // let a = serial_port.write(output).expect("write failed");
    // serial_port
}

pub fn connect() -> Result<(TTYPort, String), ArduinoStatus> {
    let port_info = find_port()?;
    let port = connect_port(&port_info.port_name)?;
    return Ok((port, port_info.port_name));
}

// pub fn listen() {
//     let mut line = String::new();
//     let delay = Duration::from_millis(100);

//     thread::spawn(move || loop {
//         println!("Suspending...");
//         match rx.recv() {
//             Ok(_) => {
//                 let len = port.read_to_string(&mut line);
//                 thread::sleep(Duration::from_millis(delay));
//             }
//             Err(_) => {
//                 println!("Terminating.");
//                 break;
//             }
//         }
//     });
// }
