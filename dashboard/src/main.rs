use eframe::{egui, NativeOptions};
use dashboard::Application;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        decorated: true,
        transparent: true,
        min_window_size: Some(egui::vec2(500.0, 500.0)),
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
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