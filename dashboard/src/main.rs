use eframe::{egui, NativeOptions, Theme};
use dashboard::Application;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        decorated: true,
        transparent: true,
        default_theme: Theme::Dark,
        min_window_size: Some(egui::vec2(1280.0, 720.0)),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Calyx",
        options,
        Box::new(|cc| {
            let app = Application::new(cc);
            Box::new(app)
        })
    )
}