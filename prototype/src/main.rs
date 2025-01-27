mod commands;

use serialport::{COMPort, SerialPort, SerialPortInfo};
use std::io::{self};

use commands::Commands;

fn send_data(port: &mut Box<dyn SerialPort>, data: &[u8]) {
    match port.write(data) {
        Err(e) => eprintln!("Failed to write to port: {}", e),
        _ => (),
    }
}

// start bit | 5 bits | parity bit (0 if even no. of 1s) | stop bit |

fn main() -> io::Result<()> {
    // Serial port configuration

    let ports = serialport::available_ports().expect("No ports found!");

    let mut actual_port: Option<Box<dyn SerialPort>> = None;

    for p in ports {
        let mut port = match serialport::new(&p.port_name, 9600)
            .timeout(std::time::Duration::from_millis(10))
            .open()
        {
            Ok(p) => p,
            Err(_) => continue,
        };

        let mut buf: Vec<u8> = vec![0; 1];

        let timeout = std::time::Instant::now();

        // port.write_data_terminal_ready(false)?;
        // std::thread::sleep(std::time::Duration::from_millis(100));
        // port.write_data_terminal_ready(true)?;

        while timeout.elapsed().as_secs() < 1 {
            let _ = port.write(&[Commands::HandshakeInit as u8]);

            if port.read(&mut buf).is_ok() {
                actual_port = Some(port);
                break;
            }
        }
    }

    if actual_port.is_none() {
        eprintln!("Failed to connect to any port!");
        return Ok(());
    }

    let mut port = actual_port.unwrap();
    send_data(&mut port, &[Commands::HandshakeInit as u8]);

    // Main loop

    let mut buffer = [0; 2];

    loop {
        match port.read_exact(&mut buffer) {
            Ok(_) => {
                let data = u16::from_be_bytes(buffer);
                println!("Received data: {}", data);
            }
            Err(_) => (),
        }
    }

    Ok(())
}
