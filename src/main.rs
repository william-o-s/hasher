mod app;

use app::AppState;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Hasher",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}
