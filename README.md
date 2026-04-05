# File Hasher

A lean, cross-platform graphical desktop utility for computing and comparing file hashes. Built with **Rust** and **egui** to provide a fast, memory-safe, and native experience without the overhead of heavy browser-based frameworks.

## Basic Flow
1. **Browse:** Click the "Browse" button to select any file from your local storage. The application uses your native OS file picker.
2. **Hash Entry:** Enter the expected SHA256 hash string (provided by the author of the file) into the text box.
3. **Verification:** As soon as the file is chosen, the application reads the file in chunks in a background thread and computes the SHA256 hash. A progress bar tracks the hashing. 
4. **Result:** Once computed, the application will provide a prominent "Match" or "No Match" indicator. 

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
