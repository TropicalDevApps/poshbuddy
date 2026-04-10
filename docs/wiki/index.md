# PoshBuddy Wiki: Home

> **Metadata**  
> **Updated**: 2026-04-10  
> **Read Time**: 3 min  
> **Difficulty**: Beginner  

PoshBuddy is a cross-platform TUI (Terminal User Interface) written in Rust, specifically designed to eliminate the friction of managing **Oh My Posh** themes and Nerd Fonts on Windows and PowerShell environments.

## Problem Statement
Customizing a PowerShell prompt typically involves manual JSON editing, multiple shell profile syncs (`$PROFILE`), and external font installations. PoshBuddy automates this entire lifecycle from a single, unified interface.

## PoshBuddy vs. Traditional Methods

| Feature | PoshBuddy TUI | Manual JSON | PowerShell Scripts |
| :--- | :--- | :--- | :--- |
| **Preview** | Real-time ANSI Render | None | Static/Broken |
| **Profile Sync** | Automatic (5.1 & 7) | Manual | Hardcoded |
| **Font Manager** | Integrated | External | None |
| **Dependencies** | Self-validating | Manual install | Prerequisite |

## 5-Minute Quickstart

### 1. Build from Source
Ensure you have [Rust](https://rustup.rs/) installed.

```powershell
PS> git clone https://github.com/julesklord/poshbuddy.git
PS> cd poshbuddy
PS> cargo run --release
```

### 2. Startup Diagnostics
On first launch, PoshBuddy will run a system check for:
- **Nerd Fonts** (Required for icons)
- **PowerShell 7** (Recommended for performance)
- **Terminal Emulator** (Windows Terminal recommended)

### 3. Choose & Apply
Use the arrows `[UP/DOWN]` to find a theme and press `[ENTER]` to apply it instantly to all your PowerShell profiles.

---
**Next Step**: [Installation & Prerequisites](installation)
