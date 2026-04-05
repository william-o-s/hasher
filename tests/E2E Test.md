# Manual E2E Test Steps

After building the release, a user can run this simple End-to-End test to verify all core features:

1. **Launch App:** Start the application. The UI should render immediately with empty states.
2. **Browse Persisted:** Click "Browse". The dialog should default to your `Downloads` directory (or Home). Cancel it. Click "Browse" again, navigate to a new directory (e.g., `Documents`), and select a file. Next time you click "Browse", it should remember `Documents`.
3. **Verify Hashing Progress:** Select a large file (e.g., a 1GB+ .iso or video). The UI should instantly display a progress bar that smoothly increments to 100% without the window freezing or showing a "Not Responding" OS state.
4. **Verify 'No Match' (Negative Case):** Type a random string like "invalidhash123" into the Expected Hash input box. The UI should instantly show a red "No Match".
5. **Verify 'Match' (Positive Case):** Copy the fully calculated SHA256 string from the bottom and paste it into the Expected Hash input box. The UI should instantly change to a green "Match".
