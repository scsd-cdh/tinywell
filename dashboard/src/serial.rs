use eframe::egui::{ComboBox, Context, Ui};
use serialport::{available_ports, SerialPortBuilder, SerialPortInfo};

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
        ui.horizontal(|ui| {
            if ui.button("connect").clicked() {

            }
            ComboBox::from_label("")
                .selected_text(format!("{}", self.port.port_name))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);

                    for port_info in &self.ports {
                        ui.selectable_value(&mut self.port, port_info.clone(), port_info.clone().port_name);
                    }
                });
        });

    }
}