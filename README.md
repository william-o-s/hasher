# File Hasher

A lean, cross-platform graphical desktop utility for computing and comparing file hashes. Built with **Rust** and **egui** to provide a fast, memory-safe, and native experience without the overhead of heavy browser-based frameworks.

## Basic Flow
1. **File Selection:** Click the "Browse" button to select a file using your native OS file picker, or simply drag and drop a file into the window.
2. **Hash Entry:** Enter the expected SHA256 hash string into the text box. The application will dynamically check for a match as you type.
3. **Verification:** Click the "Compute Hash" button to start the computation. The application reads the file in chunks in a background thread with a progress bar.
4. **Result:** The application provides a prominent "Match" or "No Match" indicator that updates in real-time.

## Convenience Features
- **Directory Retention**: Remembers the last used directory during the current run.
- **Auto-Paste**: Automatically detects and pastes a valid SHA256 hash from the clipboard on startup or file change.
- **History List**: Keeps a record of checked files with their status; clicking an item restores its state.
- **Drag-and-Drop Visual Cue**: Dims the window and shows a prompt when a file is hovered over.



## Installation and Usage

To run this application from source, you will need to install the [Rust Toolchain](https://rustup.rs/). 

**Running directly:**
```bash
# From the project root, run the application
cargo run
```

**Building a standalone executable:**
```bash
# Builds an optimized executable natively for your current OS
cargo build --release
```
Your compiled binary will be located at `target/release/hasher` (or `hasher.exe` on Windows).

## Stretch Goals / Roadmap
- **Drag and Drop:** Implement ability to drag and drop files and `.sha256` hash strings directly into the window.
- **Multiple Algorithms:** Implement a builder interface allowing users to select multiple hashing algorithms (MD5, SHA1, SHA512) to run in parallel.
- **Theming:** Automatic dark/light mode switches following the user's OS settings.

## Technical Details

This application is engineered with the following architectural design:
- **Immediate Mode GUI:** Built using `eframe`/`egui`. The UI loop repaints frames rather than relying on retained state objects. 
- **Non-blocking Hashing:** To avoid stalling the UI when processing large files, file hashing is deferred to a dedicated background `std::thread`.
- **SPSC Inter-thread Communication:** The background worker reads the file in chunks and communicates progress updates (0.0% to 100.0%) and the final hash back to the main UI thread using standard `std::sync::mpsc` channels. 
- **Repaint Context Wakeup:** As progress events are emitted over the channel, the worker invokes `Context::request_repaint()` to awaken the UI loop and ensure smooth rendering of the progress bar without wasteful polling.
- **Dependencies:** Strives for a minimal footprint using exactly what is required (`rfd` for native bindings, `sha2` for cryptographic hashing).

## Acknowledgements

This project was architected and developed with the pair-programming assistance of Antigravity, an agentic AI coding assistant.
