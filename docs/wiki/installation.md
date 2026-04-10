# PoshBuddy Wiki: Installation & Prerequisites

> **Metadata**  
> **Updated**: 2026-04-10  
> **Read Time**: 4 min  
> **Difficulty**: Beginner  

PoshBuddy is a standalone binary once compiled, but it relies on external tools to provide the "Golden Standard" terminal experience.

## Prerequisites

### 1. Developer Environment

- **Rust Toolchain**: 1.76 or higher.
- **Git**: For cloning and updating the source.

### 2. Runtime Dependencies

PoshBuddy validates these at startup:
- **Oh My Posh**: The core engine. If missing, PoshBuddy offers a 1-click install via `winget`.
- **Nerd Fonts**: Required to prevent "broken box" characters in your prompt.

## Setup Verification

When you run `poshbuddy`, the first screen is the **System Diagnostic**:

```text
  🔍 SYSTEM DIAGNOSTICS

  [ √ ] Nerd Font Detected
  [ √ ] PowerShell 7 Detected
  [ ! ] Classic Console (Windows Terminal recommended)
```

### Troubleshooting Diagnostics

- **Nerd Font [ ! ]**: Go to the **Fonts [2]** tab in PoshBuddy, choose a font (e.g., MesloLGS NF), and install it. You must then manually set this font in your terminal settings.
- **PowerShell 7 [ ! ]**: PoshBuddy works with Windows PowerShell 5.1, but it is significantly slower. We recommend installing [PowerShell 7](https://aka.ms/pscore6).
- **Classic Console [ ! ]**: If you see this, you are using `conhost.exe`. Icons and colors will be degraded. Download **Windows Terminal** from the Microsoft Store.

## Manual Dependency Install

If the automatic installation fails, you can run these commands manually:

```powershell
# Install Oh My Posh
PS> winget install JanDeDobbeleer.OhMyPosh

# Verify installation
PS> oh-my-posh --version
29.9.2
```

---
**Next Step**: [Command Reference](commands)
