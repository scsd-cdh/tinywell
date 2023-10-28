use dashboard::Application;
use eframe::{egui, NativeOptions, Theme};

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        decorated: true,
        transparent: true,
        follow_system_theme: false,
        default_theme: Theme::Dark,
        min_window_size: Some(egui::vec2(650.0, 600.0)),
        initial_window_size: Some(egui::vec2(650.0, 600.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Tinywell",
        options,
        Box::new(|cc| {
            let app = Application::new(cc);
            Box::new(app)
        }),
    )
}
