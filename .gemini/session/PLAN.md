# File Hasher Desktop App with Rust & egui

This plan outlines the architecture and steps to build a cross-platform (Windows, Linux, MacOS) desktop application for verifying file hashes using Rust and `egui`.

## Proposed Changes

### Project Setup
We will initialize a new Rust application in the current directory (`hasher`) and add the necessary dependencies to `Cargo.toml`.

#### [NEW] Cargo.toml dependencies
We will need:
- `eframe` - The official egui framework to run native desktop apps.
- `rfd` - Rust File Dialog for opening native OS file pickers seamlessly.
- `sha2` - For SHA256 hashing.
- `directories` - To reliably find the user's 'Downloads' or 'Home' directory across platforms.
- `std::sync::mpsc` - Standard library channel to communicate between the background hashing thread and the UI.

---

### UI and Application State

#### [MODIFY] src/main.rs
This will be the main entry point, bootstrapping the `eframe` application.

#### [NEW] src/app.rs
The main UI and state management for `egui`.
The application state will track:
- `selected_file_path`: Optional path to the chosen file.
- `last_used_dir`: For persisting the directory path for the `rfd` dialog.
- `expected_hash`: A string bound to the text input right side.
- `calculation_status`: An enum representing the state (Idle, Hashing(progress_float), Finished(String), Error(String)).

**UI Layout:**
- **Top/Middle:** A side-by-side layout (using `egui::Columns` or `egui::Ui::horizontal`).
  - *Left:* A "Browse" button. Clicking this triggers the `rfd` dialog.
  - *Right:* A single-line/multi-line TextEdit for the expected SHA256 hash.
- **Bottom:** Results area showing:
  - If state is `Hashing`: An `egui::ProgressBar`.
  - The calculated hash.
  - The expected hash.
  - A prominent green "Match" or red "No Match" label based on string comparison of the hashes.

---

### Hashing & Concurrency

#### [NEW] src/hasher.rs
To avoid freezing the immediate mode GUI when processing large files, we will spawn a background thread.
- When a file is selected, the app spawns a `std::thread`.
- The thread opens the file via standard Rust `std::fs::File`.
- It reads the file chunk by chunk (e.g., 8MB chunks) updating a `sha2::Sha256` hasher context.
- After every few chunks, it calculates the percentage of file read (bytes read / total file size) and sends a progress update `f32` (0.0 to 1.0) back to the UI thread via `std::sync::mpsc`.
- `eframe` provides a Context waker so the background thread can constantly tell the UI thread to redraw and show the smooth progress bar.
- Once complete, it sends the finalized String, allowing the UI to finalize its state.

## Verification Plan

### Automated/Compiler Checks
- Run `cargo check` to ensure all library interactions are sound.
- Write a simple unit test for the hashing chunking algorithm against a known string/file buffer.

### Manual Final E2E Test Steps
After building the release, a user can run this simple End-to-End test to verify the app:
1. **Launch App:** Start the application. The UI should render immediately with empty states.
2. **Browse Persisted:** Click "Browse". The dialog should default to your `Downloads` directory (or Home). Cancel it. Click "Browse" again, navigate to a new directory (e.g., `Documents`), and select a file. Next time you click "Browse", it should remember `Documents`.
3. **Verify Hashing Progress:** Select a large file (e.g., a 1GB+ .iso or video). The UI should instantly display a progress bar that smoothly increments to 100% without the window freezing or "Not Responding".
4. **Verify 'No Match':** Type a random string like "invalidhash" into the Expected Hash input box. The UI should instantly show a red "No Match".
5. **Verify 'Match':** Copy the calculated SHA256 string from the bottom and paste it into the Expected Hash input box. The UI should instantly change to a green "Match".
