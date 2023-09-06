use eframe::egui;
use eframe::egui::{Align2, Color32, Pos2, Sense, Stroke, TextStyle, Ui};
use crate::{BOX_SIDE, CELL_RADIUS};
use crate::colors::*;

#[derive(Debug, Default)]
pub struct MicroWell {
    pub led_on: bool,
    pub measurement: f32,
}

impl MicroWell {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // Allocate some space for the button
        let (response, painter) = ui.allocate_painter(egui::Vec2::new(BOX_SIDE, BOX_SIDE), Sense::click());

        // Calculate the center and radius of the circle
        let mut center = response.rect.center();

        // Choose color based on hover state
        let fill_color = if response.hovered() {
            if self.led_on {
                COLOR_BLUE_300
            } else {
                COLOR_SLATE_500
            }
        } else {
            if self.led_on {
                COLOR_BLUE_400
            } else {
                COLOR_SLATE_600
            }
        };

        // Draw the circle with the chosen color
        painter.circle_filled(center, CELL_RADIUS, fill_color);

        // Add white outline if hovered
        if response.hovered() {
            let stroke = Stroke::new(1.0, Color32::from_rgb(255, 255, 255));
            painter.circle_stroke(center, CELL_RADIUS, stroke);
        }

        // Draw text
        let text_pos = Pos2 { x: center.x, y: center.y }; // these offsets are just for example, you may have to adjust these
        painter.text(
            text_pos,
            Align2::CENTER_CENTER,
            &self.measurement,
            TextStyle::Heading.resolve(&ctx.style()),
            COLOR_SLATE_100,
        );

        // Check for interactions
        if response.clicked() {
            self.led_on = !self.led_on;
        }
    }
}