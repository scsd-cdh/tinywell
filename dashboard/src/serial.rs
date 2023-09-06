use eframe::egui::{ComboBox, Context, Sense, Stroke, Ui, Vec2};
use serialport::{available_ports, SerialPortInfo};
use crate::colors::{COLOR_BLUE_600, COLOR_ROSE_400, COLOR_WHITE};

pub struct Serial {
    ports: Vec<SerialPortInfo>,
    port: SerialPortInfo,
    is_recording: bool
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
            port,
            is_recording: false
        }
    }
}

impl Serial {
    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ComboBox::from_label("")
                .selected_text(format!("{}", self.port.port_name))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);

                    for port_info in &self.ports {
                        ui.selectable_value(&mut self.port, port_info.clone(), port_info.clone().port_name);
                    }
                });
            if ui.button("connect").clicked() {

            }
            // Allocate some space for the button
            let (response, painter) = ui.allocate_painter(Vec2::new(30.0, 30.0), Sense::click());

            // Get the rectangular region to draw the circle
            let rect = response.rect;

            // Calculate the center and radius of the circle
            let center = rect.center();
            let radius = rect.width() * 0.4;

            // Draw the circle
            if self.is_recording {
                painter.circle_filled(center, radius, COLOR_ROSE_400);
                painter.rect_filled(response.rect.shrink(11.0), 1.0, COLOR_WHITE);
            } else {
                painter.circle_filled(center, radius * 0.3, COLOR_WHITE);
            }
            let stroke = Stroke::new(1.0, COLOR_WHITE);
            painter.circle_stroke(center, radius, stroke);


            // Check for interactions
            if response.clicked() {
                self.is_recording = !self.is_recording;
            }
        });

    }
}