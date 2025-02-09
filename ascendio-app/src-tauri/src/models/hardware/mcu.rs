use std::sync::Mutex;

use log::{debug, error, info};
use serialport::{available_ports, SerialPort};

use crate::{
    errors::{Result, SerialError},
    models::Command,
};

pub struct MCU {
    pub serial_port: Option<Mutex<Box<dyn SerialPort + Send>>>,
}

impl MCU {
    pub fn new() -> Self {
        Self { serial_port: None }
    }

    pub fn send_command(&mut self, command: Command) -> Result<()> {
        if let Some(port) = &self.serial_port {
            let mut port = port.lock().unwrap();
            port.write(&command.to_bytes())?;
            debug!("Sending command: {:?}", command);
            Ok(())
        } else {
            Err(SerialError::DeviceNotFound.into())
        }
    }

    pub async fn wait_for_command(&mut self, command: Command) -> Result<()> {
        if let Some(port) = &self.serial_port {
            let mut port = port.lock().unwrap();

            // try and cast the read result to a command
            let mut buf: Vec<u8> = vec![0; 1];
            let mut command_type = Command::Unknown;

            while command_type != command {
                port.read(&mut buf)?;
                command_type = Command::from_bytes(&buf);
            }

            Ok(())
        } else {
            Err(SerialError::DeviceNotFound.into())
        }
    }

    pub fn try_connect(&mut self, baud_rate: u32) -> Result<bool> {
        let ports = available_ports()?;

        debug!("Trying to connect to MCU");
        debug!("Available ports: {:?}", ports);

        for p in ports {
            let mut port = match serialport::new(&p.port_name, baud_rate)
                .timeout(std::time::Duration::from_millis(10))
                .open()
            {
                Ok(p) => p,
                Err(_) => continue,
            };

            debug!("Trying to connect to port: {}", p.port_name);

            let now = std::time::Instant::now();

            while now.elapsed().as_secs() < 2 {
                let mut buf: Vec<u8> = vec![0; 1];

                port.write(&Command::Init.to_bytes())?;

                debug!("Sent: {:?}", Command::Init.to_bytes());

                port.read_exact(&mut buf)?;

                debug!("Received: {:?}", buf);

                if Command::from_bytes(&buf) == Command::InitAck {
                    info!("Connected to MCU on port: {}", p.port_name);
                    self.serial_port = Some(Mutex::new(port));
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    pub fn is_connected(&mut self) -> bool {
        let mut buf = vec![0; 1];
        if self.send_command(Command::Health).is_ok() {
            if let Some(port) = &self.serial_port {
                let mut port = port.lock().unwrap();

                if port.read_exact(&mut buf).is_ok() {
                    if Command::from_bytes(&buf) == Command::HealthAck {
                        return true;
                    }
                }
            }
        }

        false
    }
}
