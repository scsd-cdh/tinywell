use crate::microwell::{MicroWell, Wavelength};
use eframe::egui;

pub const BOX_SIDE: f32 = 50.0;
pub const CELL_RADIUS: f32 = BOX_SIDE * 0.4;
pub const CELL_SPACING: f32 = 40.0;
pub const MICRO_WELL_NUM: f32 = 5.0;

#[derive(Debug)]
pub struct MicroPlate {
    pub brightness: f32,
    pub wavelength: Wavelength,
    pub duration: u64,
    pub wells: Vec<MicroWell>,
}

impl Default for MicroPlate {
    fn default() -> Self {
        Self {
            brightness: 100.0,
            wavelength: Wavelength::default(),
            duration: 10,
            wells: vec![
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
                MicroWell::default(),
            ],
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
                        let idx = row as usize * 5 + col as usize;
                        if (col == 3 && row != 3)
                            || (col != 3 && row == 3)
                            || (row == 4 && col == 1)
                        {
                            self.wells[idx].disabled = true;
                        }

                        self.wells.iter_mut().for_each(|mut well| well.damaged = false);

                        if (col == 4 && row != 4) || (col == 2 && row == 0) || (col == 2 && row == 4) || (col == 0 && row == 1) {
                            self.wells[idx].damaged = true;
                        }

                        match self.wells[idx].wavelength {
                            Wavelength::W470nm => {
                                if col == 2 && row == 1 {
                                    self.wells[idx].damaged = true;
                                }
                            }
                            Wavelength::W570nm => {
                                if col == 0 && row == 0 {
                                    self.wells[idx].damaged = true;
                                }
                            }
                            Wavelength::W630nm => {
                                if (col == 4) || (col == 3) || (col == 2 && row == 1) {
                                    self.wells[idx].damaged = true;
                                }
                            }
                            Wavelength::W850nm => {
                                if (col == 2 && row == 1) || col == 0 && row == 0 {
                                    self.wells[idx].damaged = true;
                                }
                            }
                        }

                        self.wells[idx].brightness = self.brightness.clone();
                        self.wells[idx].wavelength = self.wavelength.clone();
                        self.wells[idx].show(ctx, ui);

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
