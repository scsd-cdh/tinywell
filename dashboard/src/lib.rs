#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use eframe::egui;
use eframe::epaint::{Color32};
use crate::colors::*;
use crate::serial::Serial;
use chrono::prelude::*;
use std::io::Write;
use std::ptr::write;
use crate::microplate::MicroPlate;

mod colors;
mod microplate;
mod microwell;
mod serial;

pub struct Application {
    sequence: Vec<MicroPlate>,
    current_plate: usize,
    serial: Serial,
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
            sequence: vec![MicroPlate::default()],
            current_plate: 0,
            serial: Serial::default(),
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

            write!(file, "{},{}", self.sequence[self.current_plate].brightness, self.sequence[self.current_plate].wavelength).expect("Unable to write brightness and wavelength.");
            writeln!(file, "{}", self.sequence[self.current_plate].wells.iter()
                    .map(|well| well.measurement.to_string())
                    .collect::<Vec<String>>()
                    .join(",")).expect("Unable to write line of well data");

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
                });

                if self.is_recording {
                    if ui.button("Stop Simulation").clicked() {
                        self.is_recording = false;
                    }
                } else {
                    if ui.button("Run Simulation").clicked() {
                        self.is_recording = true;
                        // Get the current time
                        let local_time: DateTime<Local> = Local::now();

                        // Format the current time as a string
                        let time_str = local_time.format("%Y-%m-%d_%H-%M-%S").to_string();

                        // Create a filename with the current time
                        self.current_file = PathBuf::from(&self.path);
                        self.current_file.push(format!("microfluidic_test_{}.csv", time_str));

                        self.last_write_time = Instant::now();
                    }
                }

            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    for (idx, plate) in self.sequence.iter().enumerate() {
                        ui.selectable_value(&mut self.current_plate, idx, format!("Plate Config {}\nduration: {}s", idx + 1, plate.duration));
                    }
                    if ui.button("New Config").clicked() {
                        self.sequence.push(MicroPlate::default());
                        self.current_plate = self.sequence.len() - 1;
                    }
                });

                self.sequence[self.current_plate].show(ctx,ui);
            });

        });

        ctx.request_repaint();
    }
}