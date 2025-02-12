use log::{debug, info};

use tokio_serial::SerialPort;

use crate::{
    errors::{Result, SerialError},
    models::Command,
};

pub struct MCU {
    pub serial_port: Option<Box<dyn SerialPort>>,
}

impl Clone for MCU {
    fn clone(&self) -> Self {
        Self {
            serial_port: match &self.serial_port {
                Some(port) => Some(port.try_clone().unwrap()),
                None => None,
            },
        }
    }
}

impl MCU {
    pub fn new() -> Self {
        Self { serial_port: None }
    }

    pub fn get_mut_serial_port(&mut self) -> Result<&mut Box<dyn SerialPort>> {
        match &mut self.serial_port {
            Some(port) => Ok(port),
            None => Err(SerialError::DeviceNotFound.into()),
        }
    }

    pub fn send_command(&mut self, command: Command) -> Result<()> {
        let port = self.get_mut_serial_port()?;

        port.write(&command.to_bytes())?;
        Ok(())
    }

    pub async fn wait_for_command(&mut self, command: Command) -> Result<()> {
        let port = self.get_mut_serial_port()?;

        let mut buf: Vec<u8> = vec![0; 1];
        let mut command_type = Command::Unknown;

        while command_type != command {
            port.read(&mut buf)?;
            command_type = Command::from_bytes(&buf);
        }

        Ok(())
    }

    pub fn try_connect(&mut self, baud_rate: u32) -> Result<bool> {
        let ports = tokio_serial::available_ports()?;

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
                    self.serial_port = Some(port);
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    pub fn disconnect(&mut self) -> Result<()> {
        self.serial_port = None;

        Ok(())
    }

    pub fn is_connected(&mut self) -> bool {
        let mut buf = vec![0; 1];

        if let Some(port) = &mut self.serial_port {
            let command = Command::Health;

            if port.write(&command.to_bytes()).is_err() {
                return false;
            }

            if port.read(&mut buf).is_err() {
                return false;
            }

            if Command::from_bytes(&buf) == Command::HealthAck {
                return true;
            }
        }

        false
    }
}
