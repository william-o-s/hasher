use crate::hasher::WorkerMessage;
use eframe::egui;
use egui::epaint;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};

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

#[derive(Clone, Debug)]
pub struct HistoryItem {
    pub file_name: String,
    pub file_path: String,
    pub computed_hash: String,
    pub status: VerificationStatus,
}

pub struct AppState {
    pub file_path: Option<String>,
    pub expected_hash: String,
    pub computed_hash: Option<String>,
    pub progress: f32,
    pub status: VerificationStatus,
    pub receiver: Option<Receiver<WorkerMessage>>,
    pub last_dir: Option<PathBuf>,
    pub clipboard_checked: bool,
    pub history: Vec<HistoryItem>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            file_path: None,
            expected_hash: String::new(),
            computed_hash: None,
            progress: 0.0,
            status: VerificationStatus::Idle,
            receiver: None,
            last_dir: None,
            clipboard_checked: false,
            history: Vec::new(),
        }
    }
}

impl AppState {
    fn render_file_selection(&mut self, ui: &mut egui::Ui, frame: egui::Frame) {
        frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("File Selection").strong());
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if ui.button("Browse").clicked() {
                        let mut dialog = rfd::FileDialog::new();
                        if let Some(ref dir) = self.last_dir {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.pick_file() {
                            let path_str = path.to_string_lossy().to_string();
                            log::info!("File selected via dialog: {}", path_str);
                            self.file_path = Some(path_str.clone());
                            self.computed_hash = None; // Clear previous result
                            self.clipboard_checked = false; // Trigger clipboard check again

                            // Store parent directory
                            if let Some(parent) = path.parent() {
                                self.last_dir = Some(parent.to_path_buf());
                            }
                        }
                    }

                    if let Some(path) = &self.file_path {
                        ui.label(path);
                    } else {
                        ui.label("No file selected");
                    }
                });
            });
        });
    }

    fn render_verification_setup(&mut self, ui: &mut egui::Ui, frame: egui::Frame) {
        frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Verification").strong());
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Expected Hash:");
                    ui.text_edit_singleline(&mut self.expected_hash);
                    self.expected_hash = transform_input_hash(&self.expected_hash);
                });

                ui.add_space(5.0);

                if self.file_path.is_some() && self.status != VerificationStatus::Hashing {
                    if ui.button("Compute Hash").clicked() {
                        let path_str = self.file_path.clone().unwrap();
                        log::info!("Starting hash computation for: {}", path_str);

                        let (sender, receiver) = channel();
                        self.receiver = Some(receiver);
                        self.status = VerificationStatus::Hashing;
                        self.progress = 0.0;

                        let ctx_clone = ui.ctx().clone();
                        std::thread::spawn(move || {
                            crate::hasher::hash_file(path_str, sender);
                            ctx_clone.request_repaint();
                        });
                    }
                }
            });
        });
    }

    fn render_results(&mut self, ui: &mut egui::Ui, frame: egui::Frame) {
        frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Status").strong());
                ui.add_space(5.0);

                ui.add(
                    egui::ProgressBar::new(self.progress)
                        .text(format!("{:.1}%", self.progress * 100.0)),
                );
                ui.add_space(5.0);

                // Dynamic match check
                if let Some(ref computed_hash) = self.computed_hash {
                    self.status = get_verification_status(computed_hash, &self.expected_hash);

                    egui::Grid::new("comparison_grid").show(ui, |ui| {
                        ui.label(egui::RichText::new("Expected Hash: ").strong());
                        ui.label(format!("{}", self.expected_hash));
                        ui.end_row();

                        ui.label(egui::RichText::new("Computed Hash: ").strong());
                        ui.label(format!("{}", computed_hash));
                        ui.end_row();
                    });
                }

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
        });
    }

    fn render_history(&mut self, ui: &mut egui::Ui, frame: egui::Frame) {
        if self.history.is_empty() {
            return;
        }

        frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("History").strong());
                ui.add_space(5.0);

                for item in &self.history {
                    ui.horizontal(|ui| {
                        if ui.link(&item.file_name).clicked() {
                            self.file_path = Some(item.file_path.clone());
                            self.computed_hash = Some(item.computed_hash.clone());
                            self.status = item.status.clone();
                            self.progress = 1.0;
                        }

                        match &item.status {
                            VerificationStatus::Match => {
                                ui.colored_label(egui::Color32::GREEN, "MATCH");
                            }
                            VerificationStatus::NoMatch => {
                                ui.colored_label(egui::Color32::RED, "NO MATCH");
                            }
                            _ => {
                                ui.label("Not verified");
                            }
                        }

                        ui.label(format!(" --> {}", item.computed_hash));
                    });
                }
            });
        });
    }

    fn render_logger(&mut self, ui: &mut egui::Ui) {
        egui_logger::logger_ui().show(ui);
    }
}

impl eframe::App for AppState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Poll for channel messages
            if let Some(ref receiver) = self.receiver {
                while let Ok(msg) = receiver.try_recv() {
                    match msg {
                        WorkerMessage::Progress(p) => {
                            self.progress = p;
                        }
                        WorkerMessage::Success(hash) => {
                            self.computed_hash = Some(hash.clone());
                            self.progress = 1.0;
                            self.status = VerificationStatus::Idle; // Computation complete

                            // Add to history
                            let status = get_verification_status(&hash, &self.expected_hash);

                            if let Some(ref path) = self.file_path {
                                let file_name = std::path::Path::new(path)
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_else(|| "Unknown".to_string());

                                self.history.push(HistoryItem {
                                    file_name,
                                    file_path: path.clone(),
                                    computed_hash: hash,
                                    status,
                                });
                            }
                        }
                        WorkerMessage::Error(e) => {
                            log::error!("Error during hash computation: {}", e);
                            self.status = VerificationStatus::Error(e);
                        }
                    }
                }
            }

            // Handle file drag-and-drop
            ui.ctx().input(|i| {
                if !i.raw.dropped_files.is_empty() {
                    if let Some(file) = i.raw.dropped_files.first() {
                        if let Some(path) = &file.path {
                            let path_str = path.to_string_lossy().to_string();
                            log::info!("File dropped: {}", path_str);
                            self.file_path = Some(path_str);
                            self.computed_hash = None; // Clear previous result
                            self.clipboard_checked = false; // Trigger clipboard check again

                            // Store parent directory
                            if let Some(parent) = path.parent() {
                                self.last_dir = Some(parent.to_path_buf());
                            }
                        }
                    }
                }
            });

            ui.add_space(10.0);
            ui.heading("File Hasher");
            ui.add_space(10.0);

            let card_frame = egui::Frame::new()
                .fill(ui.style().visuals.widgets.noninteractive.bg_fill)
                .corner_radius(8.0)
                .inner_margin(12.0);

            self.render_file_selection(ui, card_frame.clone());
            ui.add_space(10.0);
            self.render_verification_setup(ui, card_frame.clone());
            ui.add_space(10.0);
            self.render_results(ui, card_frame.clone());
            ui.add_space(10.0);
            self.render_history(ui, card_frame);
            ui.add_space(20.0);
            self.render_logger(ui);

            // Visual cue for drag-and-drop
            // Must be at the bottom to avoid drawing over main elements.
            if ui.ctx().input(|i| !i.raw.hovered_files.is_empty()) {
                let screen_rect = ui.max_rect();

                // Dim the background
                ui.painter().rect_filled(
                    screen_rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                );

                // Draw thick border
                let inset_rect = screen_rect.expand(-20.0);
                ui.painter().rect_stroke(
                    inset_rect,
                    8.0,
                    egui::Stroke::new(4.0, egui::Color32::from_rgb(65, 105, 225)),
                    epaint::StrokeKind::Middle,
                );

                // Draw text prompt in center
                ui.painter().text(
                    screen_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Drop a file here",
                    egui::FontId::proportional(24.0),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}

fn transform_input_hash(hash: &str) -> String {
    let hash = hash.strip_prefix("sha256:").unwrap_or(&hash);
    hash.to_ascii_lowercase()
}

fn get_verification_status(computed_hash: &str, expected_hash: &str) -> VerificationStatus {
    if expected_hash.is_empty() {
        VerificationStatus::Idle
    } else if computed_hash == expected_hash {
        log::info!("Hash match successful!");
        VerificationStatus::Match
    } else {
        log::warn!(
            "Hash mismatch! Expected: {}, Computed: {}",
            transform_input_hash(expected_hash),
            computed_hash
        );
        VerificationStatus::NoMatch
    }
}
