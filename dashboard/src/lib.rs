#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::colors::*;
use crate::microplate::MicroPlate;
use crate::serial::Serial;
use chrono::prelude::*;
use eframe::egui;
use eframe::epaint::Color32;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod colors;
mod microplate;
mod microwell;
mod serial;

pub struct Application {
    sequence: Vec<MicroPlate>,
    current_plate: usize,

    serial: Serial,

    folder_path: PathBuf,
    current_file: PathBuf,

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
            folder_path: PathBuf::from(
                env::current_exe()
                    .expect("Failed to get current executable path")
                    .parent()
                    .expect("Unable to find parent folder"),
            ),
            current_file: PathBuf::default(),
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
            if !well.led_on || well.damaged || well.disabled {
                continue;
            }

            req.push(
                0b10000000
                    | match idx {
                        0 => (5 * 4)    + well.wavelength.get_idx(),
                        1 => (6 * 4)    + well.wavelength.get_idx(),
                        2 =>      4     + well.wavelength.get_idx(),
                        4 => (2 * 4)    + well.wavelength.get_idx(),
                        5 => (4 * 4)    + well.wavelength.get_idx(),
                        6 => (7 * 4)    + well.wavelength.get_idx(),
                        7 =>              well.wavelength.get_idx(),
                        9 => (3 * 4)    + well.wavelength.get_idx(),
                        10 => (9 * 4)   + well.wavelength.get_idx(),
                        11 => (10 * 4)  + well.wavelength.get_idx(),
                        12 => (13 * 4)  + well.wavelength.get_idx(),
                        14 => (14 * 4)  + well.wavelength.get_idx(),
                        18 => (12 * 4)  + well.wavelength.get_idx(),
                        20 => (8 * 4)   + well.wavelength.get_idx(),
                        22 => (11 * 4)  + well.wavelength.get_idx(),
                        24 => (15 * 4)  + well.wavelength.get_idx(),
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
                self.sim_start.elapsed().as_secs()
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui
                        .button(format!("Save Directory: {:?}", self.folder_path))
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.folder_path = path;
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
            ui.separator();
            ui.add_enabled_ui(!self.is_simulating, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for (idx, plate) in self.sequence.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.current_plate,
                                idx,
                                format!("Plate Config {}\nduration: {}s", idx + 1, plate.duration),
                            );
                        }
                        if ui.button("New Plate").clicked() {
                            self.sequence.push(MicroPlate::default());
                            self.current_plate = self.sequence.len() - 1;
                        }
                    });
                    ui.vertical(|ui| {
                        ui.add_enabled_ui(self.sequence.len() > 1, |ui| {
                            if ui.button("Remove Plate").clicked() {
                                self.sequence.remove(self.current_plate);
                                if self.current_plate != 0 {
                                    self.current_plate -= 1;
                                }
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
