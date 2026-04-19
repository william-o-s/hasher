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

fn main() {
    println!("Hello, world!");
}
