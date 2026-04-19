# Project Context: File Hasher Desktop App

**Purpose:** A lean, cross-platform utility to compute and compare SHA256 file hashes. 
**Motivations:** Build a good working knowledge of Rust native GUI development.

## Tech Stack
- Language: Rust 
- GUI Framework: `eframe` (egui)
- Async/Concurrency: `std::thread`, `std::sync::mpsc`
- File Picker: `rfd`
- Hashing: `sha2`

## Core Concepts
- Uses immediate mode GUI patterns. GUI must not block.
- Background hashing is implemented using thread spawning and MPSC channels (acting as an SPSC) to relay progress (`f32` 0.0 to 1.0) and final results.
- `Context::request_repaint()` is utilized by the worker thread to force `eframe` to render progress bar updates.
- Strives for minimal dependencies and single-executable distribution without bloated frameworks.

## Conversation Notes (2026-04-19)
- **What we did:** Built a File Hasher app using `egui` and Rust. We created a separate `hasher` module for background thread processing, implemented a dynamic match check for the hash comparison, and added a 'Compute Hash' button to give the user control over when to start heavy processing. We also implemented drag-and-drop.
- **Why we did it:** To provide a lean, responsive utility that doesn't block the UI and protects user resources from accidental heavy loads. We used `egui` to learn core mechanics and keep it simple without overhead.
- **What we are doing next:** Looking at UI styling or other stretch goals (like multiple algorithms).
