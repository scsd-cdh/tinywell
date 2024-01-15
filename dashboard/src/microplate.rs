use crate::microwell::MicroWell;
use crate::wavelength::Wavelength;
use eframe::egui;
use serde::{Serialize, Deserialize};

pub const BOX_SIDE: f32 = 50.0;
pub const CELL_RADIUS: f32 = BOX_SIDE * 0.4;
pub const MICRO_WELL_NUM: i32 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroPlate {
    pub brightness: f32,
    pub wavelength: Wavelength,
    pub duration: u64,
    pub wells: Vec<MicroWell>,
}

impl Default for MicroPlate {
    fn default() -> Self {
        let mut wells = vec![];

        for i in 0u8..5 {
            let mut letter = 'D';
            for _ in 0u8..5 {
                wells.push(MicroWell::new(format!("{}{}", letter, 5-i)));
                letter = ((letter as u8) + 1) as char;
            }
        }

        Self {
            brightness: 50.0,
            wavelength: Wavelength::default(),
            duration: 5,
            wells
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

            for row in 0..MICRO_WELL_NUM {
                ui.horizontal(|ui| {
                    for col in 0..MICRO_WELL_NUM {
                        let idx = row as usize * 5 + col as usize;
                        if (col == 1 && row != 1)
                            || (col != 1 && row == 1)
                            || (row == 3 && col == 0)
                        {
                            self.wells[idx].disabled = true;
                        }

                        self.wells[idx].brightness = self.brightness;
                        self.wells[idx].wavelength = self.wavelength.clone();
                        self.wells[idx].show(ctx, ui);
                    }
                });
            }
        });
    }

    pub fn clear(&mut self) {
        for row in 0..MICRO_WELL_NUM {
            for col in 0..MICRO_WELL_NUM {
                let idx = row as usize * 5 + col as usize;
                self.wells[idx].measurement = 0.0;
            }
        }
    }
}
