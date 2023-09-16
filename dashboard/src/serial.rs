use eframe::egui::{Context, Ui};
use lazy_static::lazy_static;
use serialport::{available_ports, SerialPortInfo, SerialPortType};
use std::sync::RwLock;
use std::time::Duration;

lazy_static! {
    pub static ref COMMAND_QUEUE: RwLock<Vec<u8>> = RwLock::new(Vec::default());
}

pub struct Serial {
    ports: Vec<SerialPortInfo>,
    port_selected: usize,
}

impl Default for Serial {
    fn default() -> Self {
        let ports = available_ports().expect("Couldn't retrieve available ports.");

        Serial {
            ports,
            port_selected: 0,
        }
    }
}

impl Serial {
    pub fn get_port_name(port: &SerialPortInfo) -> String {
        match &port.port_type {
            SerialPortType::UsbPort(info) => {
                if info.vid == 0x16C0 {
                    String::from("Teensyduino")
                } else {
                    port.clone().port_name
                }
            }
            _ => port.clone().port_name,
        }
    }
    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        let port_names = self
            .ports
            .iter()
            .map(|port| Serial::get_port_name(port))
            .collect::<Vec<String>>();
        let port_name = match port_names.get(self.port_selected) {
            None => String::from("None"),
            Some(name) => name.clone(),
        };

        ui.menu_button(format!("Port: {}", port_name), |ui| {
            for (idx, port) in self.ports.iter().enumerate() {
                let device_name = Serial::get_port_name(port);

                ui.selectable_value(&mut self.port_selected, idx, format!("{}", device_name));
            }
        });
    }

    pub fn request_data(&mut self) {
        if let Some(info) = self.ports.get(self.port_selected) {
            match serialport::new(info.clone().port_name, 115_200)
                .timeout(Duration::from_millis(10))
                .open() {
                Ok(mut port) => {
                    let mut serial_buf: Vec<u8> = vec![0; 3];

                    while let Ok(_bytes_read) = port.read_exact(serial_buf.as_mut_slice()) {
                        // Use the bytes in `serial_buf`
                        let reconstructed_value: u16 = ((serial_buf[2] as u16) << 8) | (serial_buf[1] as u16);
                        println!("Read bytes: {} {}", serial_buf[0], reconstructed_value);
                    }
                }
                Err(_) => {}
            }
        }
    }
}
