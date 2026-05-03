#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod hasher;

use app::AppState;

fn main() -> eframe::Result<()> {
    egui_logger::builder()
        .show_all_categories(false)
        .init()
        .expect("Error initializing egui_logger");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "File Hasher",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(get_visuals());
            Ok(Box::new(AppState::default()))
        }),
    )
}

fn get_visuals() -> egui::Visuals {
    let mut visuals = eframe::egui::Visuals::dark();
    visuals.window_fill = eframe::egui::Color32::from_rgb(26, 27, 38);
    visuals.widgets.noninteractive.bg_fill = eframe::egui::Color32::from_rgb(36, 37, 54);
    visuals.widgets.inactive.bg_fill = eframe::egui::Color32::from_rgb(46, 48, 71);
    visuals.widgets.hovered.bg_fill = eframe::egui::Color32::from_rgb(66, 68, 101);
    visuals.widgets.active.bg_fill = eframe::egui::Color32::from_rgb(86, 88, 131);
    visuals.selection.bg_fill = eframe::egui::Color32::from_rgb(65, 105, 225);
    visuals
}
