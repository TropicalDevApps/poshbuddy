# PoshBuddy Wiki: Troubleshooting

> **Metadata**  
> **Updated**: 2026-04-10  
> **Read Time**: 5 min  
> **Difficulty**: Intermediate  

Issues organized by symptom.

## Symptom: Broken icons (Squares/Question marks)
**Cause**: You are not using a Nerd Font, or it is incorrectly configured.
- **Fix 1**: Ensure you have installed a font from the **Fonts [2]** tab.
- **Fix 2**: In Windows Terminal, go to **Settings > PowerShell > Appearance > Font face** and select the font ending in "Nerd Font" or "NF".
- **Fix 3**: If use VS Code, update `"terminal.integrated.fontFamily": "'MesloLGS NF'"`.

## Symptom: Themes not applying after "Success" message
**Cause**: The current shell session needs to be refreshed.
- **Fix 1**: Close and reopen your terminal.
- **Fix 2**: Run `. $PROFILE` to reload the config in the current window.
- **Fix 3**: Verify if you have multiple profiles. PoshBuddy tries to find all of them, but if yours is in a non-standard location, check the **Diagnostic** screen for detected paths.

## Symptom: Terminal is slow after installing many plugins
**Cause**: Large PowerShell profiles.
- **Fix**: Open your `$PROFILE` and remove unused `Import-Module` calls. PoshBuddy aims to keep the initialization line simple: `oh-my-posh init pwsh ...`.

## Symptom: "Error: Installation failed via Winget"
**Cause**: Usually a network timeout or lack of administrator permissions.
- **Fix 1**: Try running PoshBuddy as **Administrator**.
- **Fix 2**: Run `winget install JanDeDobbeleer.OhMyPosh` manually in a terminal to see the specific error code from Winget.

## FAQ

### Does PoshBuddy support WSL?
PoshBuddy is currently focused on native Windows environments (PowerShell 5.1 and 7). While it might run in WSL, profile detection and font installation are optimized for Windows.

### Can I use my own custom themes?
PoshBuddy currently fetches themes from the official Oh My Posh repository. Support for local theme folders is in the roadmap.

### Is it safe?
PoshBuddy only modifies your `$PROFILE` by adding or updating a single `oh-my-posh init` line. It creates a backup logic if it finds complex configurations.

---
**Return to**: [Home](index)
