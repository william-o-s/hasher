use std::fs::File;
use std::io::{Read, self};
use std::path::Path;
use sha2::{Sha256, Digest};
use std::sync::mpsc::Sender;

const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

/// Messages sent from the background hasher thread to the UI thread.
pub enum WorkerMessage {
    /// Progress update with a value between 0.0 and 1.0.
    Progress(f32),
    /// Hash computation completed successfully with the resulting hex string.
    Success(String),
    /// An error occurred during file reading.
    Error(String),
}

/// Hashes a file in chunks and sends progress updates and the final result over a channel.
pub fn hash_file(path_str: String, sender: Sender<WorkerMessage>) {
    let path = Path::new(&path_str);
    
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            let _ = sender.send(WorkerMessage::Error(e.to_string()));
            return;
        }
    };
    
    let total_size = match file.metadata() {
        Ok(m) => m.len(),
        Err(e) => {
            let _ = sender.send(WorkerMessage::Error(e.to_string()));
            return;
        }
    };
    
    let mut hasher = Sha256::new();
    let mut reader = file;
    let mut buffer = [0; CHUNK_SIZE];
    let mut bytes_read = 0;
    
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // End of file
            Ok(n) => {
                hasher.update(&buffer[..n]);
                bytes_read += n as u64;
                
                // Avoid division by zero if file is empty
                let progress = if total_size > 0 {
                    bytes_read as f32 / total_size as f32
                } else {
                    1.0
                };
                
                let _ = sender.send(WorkerMessage::Progress(progress));
            }
            Err(e) => {
                let _ = sender.send(WorkerMessage::Error(e.to_string()));
                return;
            }
        }
    }
    
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    let _ = sender.send(WorkerMessage::Success(hash_string));
}
