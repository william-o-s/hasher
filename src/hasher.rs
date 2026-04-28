use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
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
    let mut reader = BufReader::new(file);
    let mut buffer = [0; CHUNK_SIZE]; // 64KB chunk size
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
    let hash_string = hex::encode(result);
    let _ = sender.send(WorkerMessage::Success(hash_string));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::mpsc::channel;

    #[test]
    fn test_hash_file_success() {
        let test_file_path = "test_success.txt";
        let mut file = File::create(test_file_path).unwrap();
        file.write_all(b"hello world").unwrap();

        let (sender, receiver) = channel();

        hash_file(test_file_path.to_string(), sender);

        // The known SHA256 for "hello world"
        let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";

        let mut success = false;
        while let Ok(msg) = receiver.recv() {
            match msg {
                WorkerMessage::Progress(p) => {
                    assert!(p >= 0.0 && p <= 1.0);
                }
                WorkerMessage::Success(hash) => {
                    assert_eq!(hash, expected_hash);
                    success = true;
                }
                WorkerMessage::Error(e) => {
                    panic!("Expected success, got error: {}", e);
                }
            }
        }

        assert!(success, "Expected Success message was not received");

        // Clean up
        std::fs::remove_file(test_file_path).unwrap();
    }

    #[test]
    fn test_hash_file_not_found() {
        let (sender, receiver) = channel();

        hash_file("non_existent_file.txt".to_string(), sender);

        let mut error = false;
        while let Ok(msg) = receiver.recv() {
            match msg {
                WorkerMessage::Error(_) => {
                    error = true;
                }
                _ => {}
            }
        }

        assert!(error, "Expected Error message was not received");
    }
}
