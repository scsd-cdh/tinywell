use crate::microplate::MicroPlate;
use eframe::egui::{Context, Ui};
use serialport::{available_ports, SerialPortInfo, SerialPortType};
use std::time::Duration;

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
    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        self.ports = available_ports().expect("Couldn't retrieve available ports.");
        let port_names = self
            .ports
            .iter()
            .map(Serial::get_port_name)
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

    pub fn request_led(&mut self, buf: &[u8]) {
        if let Some(info) = self.ports.get(self.port_selected) {
            if let Ok(mut port) = serialport::new(info.clone().port_name, 115_200)
                .timeout(Duration::from_millis(10))
                .open()
            {
                if port.write(buf).is_ok() {}
            }
        }
    }

    pub fn request_data(&mut self, plate: &mut MicroPlate) {
        if let Some(info) = self.ports.get(self.port_selected) {
            if let Ok(mut port) =  serialport::new(info.clone().port_name, 115_200)
                .timeout(Duration::from_millis(10))
                .open()
            {
                let mut serial_buf: Vec<u8> = vec![0; 3];

                while let Ok(_bytes_read) = port.read_exact(serial_buf.as_mut_slice()) {
                    // Use the bytes in `serial_buf`
                    let reconstructed_value: u16 =
                        ((serial_buf[2] as u16) << 8) | (serial_buf[1] as u16);

                    let mux = serial_buf[0] & 0b10000000;
                    let idx = serial_buf[0] & 0b01111111;

                    match mux {
                        0 => match idx {
                            0 => plate.wells[9].measurement = reconstructed_value as f32,
                            1 => plate.wells[5].measurement = reconstructed_value as f32,
                            2 => plate.wells[6].measurement = reconstructed_value as f32,
                            3 => plate.wells[7].measurement = reconstructed_value as f32,
                            4 => plate.wells[0].measurement = reconstructed_value as f32,
                            5 => plate.wells[1].measurement = reconstructed_value as f32,
                            6 => plate.wells[2].measurement = reconstructed_value as f32,
                            7 => plate.wells[4].measurement = reconstructed_value as f32,
                            _ => {}
                        },
                        0b10000000 => match idx {
                            0 => plate.wells[22].measurement = reconstructed_value as f32,
                            1 => plate.wells[18].measurement = reconstructed_value as f32,
                            2 => plate.wells[24].measurement = reconstructed_value as f32,
                            3 => plate.wells[20].measurement = reconstructed_value as f32,
                            4 => plate.wells[10].measurement = reconstructed_value as f32,
                            5 => plate.wells[11].measurement = reconstructed_value as f32,
                            6 => plate.wells[12].measurement = reconstructed_value as f32,
                            7 => plate.wells[14].measurement = reconstructed_value as f32,
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
