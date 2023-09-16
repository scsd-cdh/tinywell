use std::time::Duration;
use eframe::egui;
use crate::microwell::{MicroWell, Wavelength};

pub const BOX_SIDE: f32 = 50.0;
pub const CELL_RADIUS: f32 = BOX_SIDE * 0.4;
pub const CELL_SPACING: f32 = 40.0;
pub const MICRO_WELL_NUM: f32 = 5.0;

#[derive(Debug)]
pub struct MicroPlate {
    pub brightness: f32,
    pub wavelength: Wavelength,
    pub duration: u32,
    pub wells: Vec<MicroWell>
}

impl Default for MicroPlate {
    fn default() -> Self {
        Self {
            brightness: 100.0,
            wavelength: Wavelength::default(),
            duration: 10,
            wells: vec![
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
            ]
        }
    }
}

impl MicroPlate {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("duration [s]: ");
                    ui.add(egui::DragValue::new(&mut self.duration).speed(1.0));
                    ui.end_row();

                    ui.label("wavelength: ");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.wavelength))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.wavelength, Wavelength::W470nm, "470nm");
                            ui.selectable_value(&mut self.wavelength, Wavelength::W570nm, "570nm");
                            ui.selectable_value(&mut self.wavelength, Wavelength::W630nm, "630nm");
                            ui.selectable_value(&mut self.wavelength, Wavelength::W850nm, "850nm");
                        });
                    ui.end_row();

                    ui.label("brightness: ");
                    ui.add(egui::Slider::new(&mut self.brightness, 0.0..=100.0).suffix("%"));
                    ui.end_row();
                });

            for row in 0..MICRO_WELL_NUM as i32 {
                ui.horizontal(|ui| {
                    for col in 0..MICRO_WELL_NUM as i32 {
                        if (col == 3 && row != 3) ||
                            (col != 3 && row == 3) ||
                            (row == 4 && col == 1) {
                            self.wells[row as usize * 5 + col as usize].disabled = true;
                        }
                        self.wells[row as usize * 5 + col as usize].brightness = self.brightness.clone();
                        self.wells[row as usize * 5 + col as usize].wavelength = self.wavelength.clone();
                        self.wells[row as usize * 5 + col as usize].show(ctx, ui);

                        // Add spacing between circles in the same row
                        if col < MICRO_WELL_NUM as i32 - 1 {
                            ui.add_space(CELL_SPACING - CELL_RADIUS * 2.0);
                        }
                    }
                });

                // Add spacing between rows
                if row < MICRO_WELL_NUM as i32 - 1 {
                    ui.add_space(CELL_SPACING - CELL_RADIUS * 2.0);
                }
            }
        });
    }
}