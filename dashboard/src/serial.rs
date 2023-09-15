use eframe::egui::{ComboBox, Context, Sense, Stroke, Ui, Vec2};
use serialport::{available_ports, SerialPortInfo};
use crate::colors::{COLOR_BLUE_600, COLOR_ROSE_400, COLOR_WHITE};

pub struct Serial {
    ports: Vec<SerialPortInfo>,
    port: SerialPortInfo
}

impl Default for Serial {
    fn default() -> Self {
        let ports = available_ports().expect("Couldn't retrieve available ports.");
        if ports.is_empty() {
            panic!("No available serial ports.");
        }

        let port = ports[0].clone();

        Serial {
            ports,
            port
        }
    }
}

impl Serial {
    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.menu_button(format!("Port: {}", self.port.port_name), |ui| {
            for port_info in &self.ports {
                ui.selectable_value(&mut self.port, port_info.clone(), port_info.clone().port_name);
            }
        });
    }
}