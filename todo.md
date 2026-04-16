# PoshBuddy Roadmap & TODO

## ✅ Completed (v0.3.4)
- [x] **New Era Redesign**: Premium block-based tab navigation.
- [x] **Stable Preview Engine**: Fixed Windows environment inheritance in `oh-my-posh`.
- [x] **Zombie Process Protection**: Implemented `kill_on_drop(true)` for auto-cleaning detached previews.
- [x] **Modern Visual Hierarchy**: High-contrast active states and Nerd Font icon integration.
- [x] **Versioning**: Synchronized manifest and changelog to v0.3.4.

## 🛠️ High Priority (Next - v0.3.5)
- [ ] **Fix Theme Infinite Loop**: Decouple `ThemeDownloaded` from `ApplyTheme` to prevent automatic resets during installation.
- [ ] **Progress Granularity**: Improve the progress bar steps (Download -> Verify -> Backup -> Apply) to reflect real status.
- [ ] **Dependency Audit**: Review `Cargo.toml` for potential size optimizations.
- [ ] **Manual Backup Modal**: Add a quick confirmation or success notification when manual backups are triggered via hotkey.

## 🚀 Future Vision
- [ ] **Theme Customizer**: Visual editor for basic OMP segment colors.
- [ ] **Online Theme Gallery**: Improved fetching and search for community themes.
- [ ] **Multi-Shell Support**: Extend profile management to Zsh/Bash/Fish (Windows-first currently).
- [ ] **Auto-Updater**: Self-update mechanism for the PoshBuddy binary.
