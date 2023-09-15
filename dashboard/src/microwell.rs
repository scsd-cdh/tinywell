use std::fmt;
use std::fmt::Formatter;
use eframe::egui;
use eframe::egui::{Align2, Color32, Pos2, Sense, Stroke, TextStyle, Ui};
use crate::{BOX_SIDE, CELL_RADIUS};
use crate::colors::*;

#[derive(Debug, Clone)]
pub enum Wavelength {
    W470nm,
    W570nm,
    W630nm,
    W850nm,
}

impl fmt::Display for Wavelength {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Wavelength::W470nm => write!(f, "470nm"),
            Wavelength::W570nm => write!(f, "570nm"),
            Wavelength::W630nm => write!(f, "630nm"),
            Wavelength::W850nm => write!(f, "850nm"),
        }
    }
}

impl Wavelength {
    pub fn get_hovered_color(&self) -> Color32 {
        match self {
            Wavelength::W470nm => {
                COLOR_BLUE_300
            }
            Wavelength::W570nm => {
                COLOR_EMERALD_300
            }
            Wavelength::W630nm => {
                COLOR_ORANGE_300
            }
            Wavelength::W850nm => {
                COLOR_RED_300
            }
        }
    }

    pub fn get_color(&self) -> Color32 {
        match self {
            Wavelength::W470nm => {
                COLOR_BLUE_400
            }
            Wavelength::W570nm => {
                COLOR_EMERALD_400
            }
            Wavelength::W630nm => {
                COLOR_ORANGE_400
            }
            Wavelength::W850nm => {
                COLOR_RED_400
            }
        }
    }
}

impl Default for Wavelength {
    fn default() -> Self {
        Wavelength::W470nm
    }
}

#[derive(Debug)]
pub struct MicroWell {
    pub led_on: bool,
    pub measurement: f32,
    pub disabled: bool,
    pub wavelength: Wavelength,
    pub brightness: f32
}

impl Default for MicroWell {
    fn default() -> Self {
        Self {
            led_on: false,
            measurement: 200.34,
            disabled: false,
            wavelength: Wavelength::default(),
            brightness: 100.0
        }
    }
}

impl MicroWell {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // Allocate some space for the button
        let (response, painter) = ui.allocate_painter(egui::Vec2::new(BOX_SIDE, BOX_SIDE), Sense::click());

        // Calculate the center and radius of the circle
        let mut center = response.rect.center();

        // Choose color based on hover state
        let fill_color = if self.disabled {
            COLOR_SLATE_700
        } else {
            if response.hovered(){
                if self.led_on {
                    let color = self.wavelength.get_hovered_color();
                    Color32::from_rgba_unmultiplied(color.r(),color.g(), color.b(), (50.0 + (self.brightness/100.0) * 205.0) as u8)
                } else {
                    COLOR_SLATE_500
                }
            } else {
                if self.led_on {
                    let color = self.wavelength.get_color();
                    Color32::from_rgba_unmultiplied(color.r(),color.g(), color.b(), (50.0 + (self.brightness/100.0) * 205.0) as u8)
                } else {
                    COLOR_SLATE_600
                }
            }
        };

        // Draw the circle with the chosen color
        painter.circle_filled(center, CELL_RADIUS, fill_color);

        // Add white outline if hovered
        if response.hovered() && !self.disabled {
            let stroke = Stroke::new(1.0, Color32::from_rgb(255, 255, 255));
            painter.circle_stroke(center, CELL_RADIUS, stroke);
        }

        if self.disabled {
            return
        }
        // Draw text
        let text_pos = Pos2 { x: center.x, y: center.y }; // these offsets are just for example, you may have to adjust these
        painter.text(
            text_pos,
            Align2::CENTER_CENTER,
            &self.measurement,
            TextStyle::Small.resolve(&ctx.style()),
            COLOR_SLATE_100,
        );

        // Check for interactions
        if response.clicked() {
            self.led_on = !self.led_on;
        }
    }
}