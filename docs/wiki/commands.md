# PoshBuddy Wiki: Command Reference

> **Metadata**  
> **Updated**: 2026-04-10  
> **Read Time**: 2 min  
> **Difficulty**: Beginner  

PoshBuddy uses a keyboard-centric interface designed for speed. There are no submenus; everything is accessible via global hotkeys.

## Global Navigation

| Key | Action |
| :--- | :--- |
| `[TAB]` | Cycle focus between the List (left) and Panel (right/bottom) |
| `[1]` | Jump to **Themes Explorer** |
| `[2]` | Jump to **Fonts Manager** |
| `[3]` | Jump to **Plugins** (Upcoming Feature) |
| `[Q / ESC]` | Exit the application |

## View-Specific Controls

### Themes Explorer [1]
- `[UP / DOWN]`: Navigate the theme list. This triggers an asynchronous preview.
- `[ANY CHAR]`: Start typing to filter themes by name (e.g., typing "rob" finds "robbyrussell").
- `[BACKSPACE]`: Remove characters from the search filter.
- `[ENTER]`: Apply the selected theme to all detected PowerShell profiles.

### Fonts Manager [2]
- `[UP / DOWN]`: Navigate the list of recommended Nerd Fonts.
- `[ENTER]`: Start the downloader and installer for the selected font.
- `[ANY CHAR]`: Filter fonts by name.

## State Transitions
1. **Startup**: `Onboarding` -> `Loading` (Fetching GitHub themes) -> `Main`.
2. **Action**: `Main` -> `Installing` (Async) -> `Success/Feedback`.
3. **Recovery**: `Error` screen allows you to pulse `Q` to exit and check logs.

ℹ️ **Info**: If you apply a theme, PoshBuddy will automatically detect both Windows PowerShell (5.1) and PowerShell Core (7) profiles and update them simultaneously.

---
**Next Step**: [Usage Patterns](usage)
