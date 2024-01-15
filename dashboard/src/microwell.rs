use crate::colors::*;
use crate::microplate::{BOX_SIDE, CELL_RADIUS};
use eframe::egui;
use eframe::egui::{Align2, Color32, Pos2, Sense, Stroke, TextStyle, Ui};
use crate::wavelength::Wavelength;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroWell {
    pub led_on: bool,
    pub measurement: f32,
    pub disabled: bool,
    pub wavelength: Wavelength,
    pub brightness: f32,
    pub label: String
}

impl Default for MicroWell {
    fn default() -> Self {
        Self {
            led_on: true,
            measurement: 0.0,
            disabled: false,
            wavelength: Wavelength::default(),
            brightness: 50.0,
            label: "A1".to_string()
        }
    }
}

impl MicroWell {
    pub fn new(label: String) -> MicroWell {
        MicroWell {
            led_on: true,
            measurement: 0.0,
            disabled: false,
            wavelength: Wavelength::default(),
            brightness: 100.0,
            label
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) -> bool {
        // Allocate some space for the button
        let (response, painter) =
            ui.allocate_painter(egui::Vec2::new(BOX_SIDE, BOX_SIDE + 15.0), Sense::click());

        // Calculate the center and radius of the circle
        let center = Pos2 {
            x: response.rect.center().x,
            y: response.rect.center().y - 10.0
        };

        // Choose color based on hover state
        let fill_color = if self.disabled {
            COLOR_SLATE_700
        } else if response.hovered() {
            if self.led_on {
                let color = self.wavelength.get_hovered_color();
                Color32::from_rgba_unmultiplied(
                    color.r(),
                    color.g(),
                    color.b(),
                    (50.0 + (self.brightness / 100.0) * 205.0) as u8,
                )
            } else {
                COLOR_SLATE_500
            }
        } else if self.led_on {
            let color = self.wavelength.get_color();
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                (50.0 + (self.brightness / 100.0) * 205.0) as u8,
            )
        } else {
            COLOR_SLATE_600
        };

        // Draw the circle with the chosen color
        painter.circle_filled(center, CELL_RADIUS, fill_color);

        // Add white outline if hovered
        if response.hovered() && !self.disabled {
            let stroke = Stroke::new(1.0, Color32::from_rgb(255, 255, 255));
            painter.circle_stroke(center, CELL_RADIUS, stroke);
        }

        if !self.disabled {
            // Draw text
            let text_pos = Pos2 {
                x: center.x,
                y: center.y,
            }; // these offsets are just for example, you may have to adjust these
            painter.text(
                text_pos,
                Align2::CENTER_CENTER,
                self.measurement,
                TextStyle::Small.resolve(&ctx.style()),
                COLOR_SLATE_100,
            );
        }


        let label_pos = Pos2 {
            x: center.x,
            y: center.y + 30.0,
        };
        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            self.label.clone(),
            TextStyle::Small.resolve(&ctx.style()),
            COLOR_SLATE_100,
        );

        // Check for interactions
        if response.clicked() {
            self.led_on = !self.led_on;
            return true;
        }

        false
    }
}
