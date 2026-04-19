use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use crate::hasher::{self, WorkerMessage};


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

pub struct AppState {
    pub file_path: Option<String>,
    pub expected_hash: String,
    pub computed_hash: Option<String>,
    pub progress: f32,
    pub status: VerificationStatus,
    pub receiver: Option<Receiver<WorkerMessage>>,
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
        }
    }
}


impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                            if hash == self.expected_hash {
                                self.status = VerificationStatus::Match;
                            } else {
                                self.status = VerificationStatus::NoMatch;
                            }
                        }
                        WorkerMessage::Error(e) => {
                            self.status = VerificationStatus::Error(e);
                        }
                    }
                }
            }

            ui.heading("File Hasher");

            
            ui.horizontal(|ui| {
                ui.label("File:");
                if ui.button("Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let path_str = path.to_string_lossy().to_string();
                        self.file_path = Some(path_str.clone());
                        
                        let (sender, receiver) = channel();
                        self.receiver = Some(receiver);
                        self.status = VerificationStatus::Hashing;
                        self.progress = 0.0;
                        
                        let ctx_clone = ctx.clone();
                        std::thread::spawn(move || {
                            crate::hasher::hash_file(path_str, sender);
                            ctx_clone.request_repaint();
                        });
                    }
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
