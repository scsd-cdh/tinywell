#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use eframe::egui::pos2;
use eframe::epaint::{Color32};
use crate::colors::*;
use crate::microwell::MicroWell;
use crate::serial::Serial;

pub const BOX_SIDE: f32 = 50.0;
pub const CELL_RADIUS: f32 = BOX_SIDE * 0.4;
pub const CELL_SPACING: f32 = 40.0;
pub const MICRO_WELL_NUM: f32 = 5.0;

mod colors;
mod microwell;
mod serial;

pub struct Application {
    well_state: Vec<MicroWell>,
    serial: Serial
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
            serial: Serial::default()
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Pattern").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Preferences").clicked() {
                        ui.close_menu();
                    }
                    ui.menu_button("Load Pattern", |ui| {
                        ui.button("1");
                        ui.button("2");
                        ui.button("3");
                    });
                });
                ui.menu_button("Tools", |ui| {
                    self.serial.show(ctx, ui);
                });

                if ui.button("Run").clicked() {

                }
            });
            ui.separator();

            for row in 0..MICRO_WELL_NUM as i32 {
                ui.horizontal(|ui| {
                    for col in 0..MICRO_WELL_NUM as i32 {
                        if (col == 3 && row != 3) ||
                            (col != 3 && row == 3) ||
                            (row == 4 && col == 1) {
                            self.well_state[row as usize * 5 + col as usize].disabled = true;
                        }
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