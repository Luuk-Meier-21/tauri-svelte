use std::io::{BufRead, BufReader};

use std::thread;

use serialport::{SerialPortInfo, SerialPortType, TTYPort};

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArduinoStatus {
    pub code: u32,
    pub message: &'static str,
}

pub fn available_ports() -> Result<Vec<SerialPortInfo>, ArduinoStatus> {
    match serialport::available_ports() {
        Ok(ports) => Ok(ports),
        Err(err) => Err(ArduinoStatus {
            code: 404,
            message: "No ports found",
        }),
    }
}

fn find_arduino_port_from(ports: Vec<SerialPortInfo>) -> Result<SerialPortInfo, ArduinoStatus> {
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
    let available_ports = available_ports()?;
    let target_port = find_arduino_port_from(available_ports.clone())?;

    Ok(target_port)
}

pub fn connect_by_path(port_path: &str) -> Result<TTYPort, ArduinoStatus> {
    match TTYPort::open(&serialport::new(port_path, 57600)) {
        Ok(port) => Ok(port),
        Err(_err) => Err(ArduinoStatus {
            code: 400,
            message: "Unable to open port",
        }),
    }
}
