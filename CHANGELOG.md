# File Hasher Desktop App with Rust & egui

This document records the localized iteration of our implementation plan.

## Architecture Highlights
- **Framework:** `eframe` (egui) for lightweight, immediate mode cross-platform UI.
- **Background Task:** `std::thread` reading chunks of bytes, updating a `sha2::Sha256` hasher context.
- **Communication:** `std::sync::mpsc` channel sending progress updates `f32` to the main UI thread.
- **State Mgmt:** Main thread listens to the channel and issues a `Context::request_repaint()` to smoothly animate standard `egui::ProgressBar` without UI freezes.

## Completed Items
- Set up project with `eframe`, `rfd`, and `sha2` dependencies.
- Created `AppState` and UI layout in `src/app.rs`.
- Implemented background file hashing in `src/hasher.rs` with progress updates.
- Added dynamic match/no match checks.
- Modified flow to use explicit "Compute Hash" button to prevent accidental load.
- Added drag-and-drop support for files.
- Refactored `update` function in `app.rs` by extracting rendering logic to separate methods.
- Added unit tests for hashing logic in `hasher.rs`.
- Implemented directory retention during the current run.
- Implemented auto-paste from clipboard for valid SHA256 hashes.
- Implemented an interactive history list of checked files.
- Added a visual cue (overlay and border) for drag-and-drop interaction.


## Verification
- Unit tests covering positive matching, and negative bad-input scenarios for the hashing engine.
- E2E testing for the user interface described in `TEST.md`.
