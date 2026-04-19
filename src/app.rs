use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    Idle,
    Hashing,
    Match,
    NoMatch,
    Error(String),
}

impl Default for VerificationStatus {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Default)]
pub struct AppState {
    pub file_path: Option<String>,
    pub expected_hash: String,
    pub computed_hash: Option<String>,
    pub progress: f32,
    pub status: VerificationStatus,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Hasher");
            
            ui.horizontal(|ui| {
                ui.label("File:");
                if ui.button("Browse").clicked() {
                    // TODO: Handle file browsing
                }

                if let Some(path) = &self.file_path {
                    ui.label(path);
                } else {
                    ui.label("No file selected");
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Expected Hash:");
                ui.text_edit_singleline(&mut self.expected_hash);
            });
            
            ui.add(egui::ProgressBar::new(self.progress).text(format!("{:.1}%", self.progress * 100.0)));
            
            match &self.status {
                VerificationStatus::Idle => {
                    ui.label("Status: Idle");
                }
                VerificationStatus::Hashing => {
                    ui.label("Status: Hashing...");
                }
                VerificationStatus::Match => {
                    ui.colored_label(egui::Color32::GREEN, "MATCH");
                }
                VerificationStatus::NoMatch => {
                    ui.colored_label(egui::Color32::RED, "NO MATCH");
                }
                VerificationStatus::Error(e) => {
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", e));
                }
            }
        });
    }
}
