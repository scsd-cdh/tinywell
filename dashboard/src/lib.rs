#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use eframe::egui;
use eframe::epaint::{Color32};
use crate::colors::*;
use crate::microwell::{MicroWell, Wavelength};
use crate::serial::Serial;
use chrono::prelude::*;
use std::io::Write;

pub const BOX_SIDE: f32 = 50.0;
pub const CELL_RADIUS: f32 = BOX_SIDE * 0.4;
pub const CELL_SPACING: f32 = 40.0;
pub const MICRO_WELL_NUM: f32 = 5.0;

mod colors;
mod microwell;
mod serial;

pub struct Application {
    well_state: Vec<MicroWell>,
    serial: Serial,
    wavelength: Wavelength,
    brightness: f32,
    path: PathBuf,
    is_recording: bool,
    current_file: PathBuf,
    last_write_time: Instant
}

impl Application {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        // Get a mutable reference to the eGUI context's style
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        style.visuals.override_text_color = Some(COLOR_SLATE_100);

        cc.egui_ctx.set_style(style);

        Self {
            well_state: vec![
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
                MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(), MicroWell::default(),
            ],
            serial: Serial::default(),
            wavelength: Wavelength::default(),
            brightness: 100.0,
            path: PathBuf::from(env::current_exe().expect("Failed to get current executable path").parent().expect("Unable to find parent folder")),
            is_recording: false,
            current_file: PathBuf::default(),
            last_write_time: Instant::now()
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let elapsed_time = self.last_write_time.elapsed();
        if self.is_recording && elapsed_time >= Duration::from_secs(1) {
            // Open file in append mode, or create it if it doesn't exist
            println!("{:?}", self.current_file);
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&self.current_file).expect("Unable to open file.");

            writeln!(file, "{}", self.well_state.iter()
                    .filter(|well| well.led_on)
                    .map(|well| well.measurement.to_string())
                    .collect::<Vec<String>>()
                    .join(",")).expect("Unable to write line");

            // Update the last write time
            self.last_write_time = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button(format!("Save Directory: {:?}", self.path)).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.path = path;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Tools", |ui| {
                    self.serial.show(ctx, ui);
                    ui.menu_button("Wavelength", |ui| {
                        if ui.button("470nm").clicked() {
                            self.wavelength = Wavelength::W470nm;
                            ui.close_menu();
                        }
                        if ui.button("570nm").clicked() {
                            self.wavelength = Wavelength::W570nm;
                            ui.close_menu();
                        }
                        if ui.button("630nm").clicked() {
                            self.wavelength = Wavelength::W630nm;
                            ui.close_menu();
                        }
                        if ui.button("850nm").clicked() {
                            self.wavelength = Wavelength::W850nm;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Presets", |ui| {
                        if ui.button("none").clicked() {
                            self.well_state.iter_mut().for_each(|well| well.led_on = false);
                        }
                        if ui.button("corners").clicked() {
                            self.well_state.iter_mut().for_each(|well| well.led_on = false);
                            self.well_state[0].led_on = true;
                            self.well_state[(MICRO_WELL_NUM - 1.0) as usize].led_on = true;
                            self.well_state[(MICRO_WELL_NUM - 1.0) as usize * 5].led_on = true;
                            self.well_state[(MICRO_WELL_NUM - 1.0) as usize * 5 + (MICRO_WELL_NUM - 1.0) as usize].led_on = true;
                        }
                        if ui.button("3x3").clicked() {
                            self.well_state.iter_mut().for_each(|well| well.led_on = false);
                            for row in 0..3 {
                                for col in 0..3 {
                                    self.well_state[row as usize * 5 + col as usize].led_on = true;
                                }
                            }
                        }
                    });
                });

                if ui.button("Upload").clicked() {

                }

                if self.is_recording {
                    if ui.button("Stop Recording").clicked() {
                        self.is_recording = false;
                    }
                } else {
                    if ui.button("Record").clicked() {
                        self.is_recording = true;
                        // Get the current time
                        let local_time: DateTime<Local> = Local::now();

                        // Format the current time as a string
                        let time_str = local_time.format("%Y-%m-%d_%H-%M-%S").to_string();

                        // Create a filename with the current time
                        self.current_file = PathBuf::from(&self.path);
                        self.current_file.push(format!("microfluidic_test_{}_{}_{}.csv", self.wavelength.to_string(), self.brightness.to_string(), time_str));

                        self.last_write_time = Instant::now();
                    }
                }

            });
            ui.separator();
            ui.add(egui::Slider::new(&mut self.brightness, 0.0..=100.0).suffix("%"));

            for row in 0..MICRO_WELL_NUM as i32 {
                ui.horizontal(|ui| {
                    for col in 0..MICRO_WELL_NUM as i32 {
                        if (col == 3 && row != 3) ||
                            (col != 3 && row == 3) ||
                            (row == 4 && col == 1) {
                            self.well_state[row as usize * 5 + col as usize].disabled = true;
                        }
                        self.well_state[row as usize * 5 + col as usize].brightness = self.brightness.clone();
                        self.well_state[row as usize * 5 + col as usize].wavelength = self.wavelength.clone();
                        self.well_state[row as usize * 5 + col as usize].show(ctx, ui);

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

        ctx.request_repaint();
    }
}