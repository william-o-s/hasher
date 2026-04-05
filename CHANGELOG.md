# File Hasher Desktop App with Rust & egui

This document records the localized iteration of our implementation plan.

## Architecture Highlights
- **Framework:** `eframe` (egui) for lightweight, immediate mode cross-platform UI.
- **Background Task:** `std::thread` reading chunks of bytes, updating a `sha2::Sha256` hasher context.
- **Communication:** `std::sync::mpsc` channel sending progress updates `f32` to the main UI thread.
- **State Mgmt:** Main thread listens to the channel and issues a `Context::request_repaint()` to smoothly animate standard `egui::ProgressBar` without UI freezes.

## Verification
- Unit tests covering positive matching, and negative bad-input scenarios for the hashing engine.
- E2E testing for the user interface described in `TEST.md`.
