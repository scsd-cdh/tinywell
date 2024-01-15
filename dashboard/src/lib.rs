#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod colors;
mod microplate;
mod microwell;
mod serial;
pub mod wavelength;
pub mod config;

use self::colors::*;
use self::microplate::MicroPlate;
use self::serial::Serial;
use chrono::prelude::*;
use eframe::egui;
use eframe::epaint::Color32;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use crate::config::{get_results_directory, load_sequence, save_sequence_as, set_results_directory};

pub struct Application {
    sequence: Vec<MicroPlate>,
    current_plate: usize,

    serial: Serial,

    folder_path: PathBuf,
    current_file: PathBuf,

    sequence_file: Option<PathBuf>,

    sim_start: Instant,
    sequence_start: Instant,
    last_write_time: Instant,
    is_simulating: bool,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        // Get a mutable reference to the eGUI context's style
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        style.visuals.override_text_color = Some(COLOR_SLATE_100);

        cc.egui_ctx.set_style(style);

        Self {
            sequence: vec![MicroPlate::default()],
            current_plate: 0,
            serial: Serial::default(),
            folder_path: get_results_directory(),
            current_file: PathBuf::default(),
            sequence_file: None,
            sim_start: Instant::now(),
            sequence_start: Instant::now(),
            last_write_time: Instant::now(),
            is_simulating: false,
        }
    }
    pub fn clear_leds(&mut self) {
        let req = vec![0b11111111];
        self.serial.request_led(req.as_slice());
    }
    pub fn request_leds(&mut self) {
        // Send simulation setup
        let mut req = vec![0b11111111];
        req.push((255.0 + (-255.0 * self.sequence[self.current_plate].brightness / 100.0)) as u8);

        for (idx, well) in self.sequence[self.current_plate].wells.iter().enumerate() {
            if !well.led_on || well.disabled {
                continue;
            }

            req.push(
                0b10000000
                    | match idx {
                        0 =>  (15 * 4)  + well.wavelength.to_u8(),
                        2 =>  (14 * 4)  + well.wavelength.to_u8(),
                        3 =>  (3 * 4)   + well.wavelength.to_u8(),
                        4 =>  (2 * 4)   + well.wavelength.to_u8(),
                        6 =>  (12 * 4)  + well.wavelength.to_u8(),
                        10 => (11 * 4)  + well.wavelength.to_u8(),
                        12 => (13 * 4)  + well.wavelength.to_u8(),
                        13 => (0 * 4)   + well.wavelength.to_u8(),
                        14 => (1 * 4)   + well.wavelength.to_u8(),
                        17 => (10 * 4)  + well.wavelength.to_u8(),
                        18 => (7 * 4)   + well.wavelength.to_u8(),
                        19 => (6 * 4)   + well.wavelength.to_u8(),
                        20 => (8 * 4)   + well.wavelength.to_u8(),
                        22 => (9 * 4)   + well.wavelength.to_u8(),
                        23 => (4 * 4)   + well.wavelength.to_u8(),
                        24 => (5 * 4)   + well.wavelength.to_u8(),
                        _ => 0b00000000,
                    },
            );
        }

        self.serial.request_led(req.as_slice());
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.serial
            .request_data(&mut self.sequence[self.current_plate]);

        if self.is_simulating
            && self.sequence_start.elapsed()
                >= Duration::from_secs(self.sequence[self.current_plate].duration)
        {
            if self.current_plate + 1 >= self.sequence.len() {
                self.is_simulating = false;
                self.clear_leds();
            } else {
                self.current_plate += 1;
                self.sequence_start = Instant::now();
                self.request_leds();
            }
        }

        if self.is_simulating && self.last_write_time.elapsed() >= Duration::from_secs(1) {
            // Open file in append mode, or create it if it doesn't exist
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&self.current_file)
                .expect("Unable to open file.");

            write!(
                file,
                "{},{},{},",
                self.sequence[self.current_plate].brightness,
                self.sequence[self.current_plate].wavelength,
                self.sequence[self.current_plate].duration
            )
            .expect("Unable to write brightness and wavelength.");
            writeln!(
                file,
                "{}",
                self.sequence[self.current_plate]
                    .wells
                    .iter()
                    .filter(|well| !well.disabled)
                    .map(|well| well.measurement.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .expect("Unable to write line of well data");

            // Update the last write time
            self.last_write_time = Instant::now();
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Sequence").clicked() {
                        if let Some(path) = &self.sequence_file {
                            save_sequence_as(path.clone(), self.sequence.clone());
                        } else if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("file_name.json")
                            .add_filter("JSON", &["json"])
                            .save_file() {
                            self.sequence_file = Some(path.clone());

                            save_sequence_as(path, self.sequence.clone());
                            ui.close_menu();
                        }
                    }

                    if ui.button("Save Sequence As").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("file_name.json")
                            .add_filter("JSON", &["json"])
                            .save_file() {
                            self.sequence_file = Some(path.clone());

                            save_sequence_as(path, self.sequence.clone());
                            ui.close_menu();
                        }
                    }

                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.sequence = load_sequence(path.clone());
                            self.sequence_file = Some(path);
                            ui.close_menu();
                        }
                    }

                    if ui.add (
                        egui::Button::new("Results Directory")
                            .shortcut_text(self.folder_path.display().to_string())
                    ).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.folder_path = path;
                            set_results_directory(self.folder_path.clone());
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Tools", |ui| {
                    self.serial.show(ctx, ui);
                });

                if self.is_simulating {
                    if ui.button("Stop Simulation").clicked() {
                        self.clear_leds();
                        self.is_simulating = false;
                    }
                } else if ui.button("Run Simulation").clicked() {
                    self.is_simulating = true;
                    self.current_plate = 0;

                    self.request_leds();
                    self.sequence[0].clear();

                    // Get the current time
                    let local_time: DateTime<Local> = Local::now();

                    // Format the current time as a string
                    let time_str = local_time.format("%Y-%m-%d_%H-%M-%S").to_string();

                    // Create a filename with the current time
                    self.current_file = PathBuf::from(&self.folder_path);
                    self.current_file
                        .push(format!("microfluidic_test_{}.csv", time_str));

                    self.last_write_time = Instant::now();
                    self.sequence_start = Instant::now();
                    self.sim_start = Instant::now();

                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&self.current_file)
                        .expect("Unable to open file.");
                    write!(
                        file,
                        "Brightness [%],Wavelength [nm],Duration [s],"
                    )
                        .expect("Unable to write file header.");
                    writeln!(
                        file,
                        "{}",
                        self.sequence[self.current_plate]
                            .wells
                            .iter()
                            .filter(|well| !well.disabled)
                            .map(|well| well.label.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                        .expect("Unable to write file header.");
                }

                if self.is_simulating {
                    let sim_duration: u64 = self.sim_start.elapsed().as_secs();
                    let total_duration: u64 =
                        self.sequence.iter().map(|plate| plate.duration).sum();

                    ui.label(format!("{} seconds left", total_duration - sim_duration));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled_ui(!self.is_simulating, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui|{
                        if ui.button("New Pattern").clicked() {
                            self.sequence.push(MicroPlate::default());
                            self.current_plate = self.sequence.len() - 1;
                        }

                        ui.add_space(5.0);

                        egui::ScrollArea::vertical()
                            .min_scrolled_height(325.0 + 80.0)
                            .show(ui, |ui|{
                                for (idx, plate) in self.sequence.iter().enumerate() {
                                    ui.selectable_value(
                                        &mut self.current_plate,
                                        idx,
                                        format!("Well Pattern {}\nsequence duration: {}s\nwavelength: {}\nbrightness: {}%", idx + 1, plate.duration, plate.wavelength, plate.brightness),
                                    );
                                }
                            });
                    });

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(self.sequence.len() > 1, |ui| {
                                if ui.button("Remove Pattern").clicked() {
                                    self.sequence.remove(self.current_plate);
                                    if self.current_plate != 0 {
                                        self.current_plate -= 1;
                                    }
                                }
                            });
                            if ui.button("Deselect All").clicked() {
                                self.sequence[self.current_plate].wells.iter_mut().for_each(|v| v.led_on = false);
                            }

                            if ui.button("Select All").clicked() {
                                self.sequence[self.current_plate].wells.iter_mut().for_each(|v| v.led_on = true);
                            }
                        });

                        self.sequence[self.current_plate].show(ctx, ui);
                    })
                });
            });
        });

        ctx.request_repaint();
    }
}
