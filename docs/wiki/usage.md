# PoshBuddy Wiki: Usage Patterns

> **Metadata**  
> **Updated**: 2026-04-10  
> **Read Time**: 5 min  
> **Difficulty**: Intermediate  

How to use PoshBuddy in real-world scenarios.

## Scenario 1: Fresh Machine Setup

You just installed Windows and want your terminal to look professional immediately.

1. Open **Windows Terminal**.
2. Clone and run PoshBuddy.
3. The **System Diagnostic** will show `oh-my-posh` is missing.
4. Press `[ENTER]`. Watch the real-time installation logs.
5. PoshBuddy reloads. Go to Tab **[2] Fonts**, select `MesloLGS NF` and install it.
6. Close the terminal, reopen it, and set `MesloLGS NF` in terminal settings.
7. Run `poshbuddy` again, pick any theme (e.g., `agnoster`), and press `[ENTER]`.
8. Total time: < 3 minutes.

## Scenario 2: Synchronizing Multi-Disk Profiles

Many users relocate their "Documents" folder to a different drive (e.g., `D:\Documents`). PoshBuddy handles this by querying the live shell instead of guessing paths.

```powershell
# PoshBuddy executes this internally to find YOUR path
PS> Write-Host -NoNewline $PROFILE
# Output: D:\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
```

If you have both PowerShell 5 and 7 installed, PoshBuddy will detect both and apply your theme to both files simultaneously, ensuring a consistent aesthetic regardless of which shell you launch.

## Scenario 3: Theme Discovery

Using the real-time ANSI preview to find themes that match your terminal's color scheme.

1. Press `[1]` to enter Themes.
2. Start typing "blue" in the filter.
3. Navigate through results. Observe the **Prompt Visual Preview** box.
4. PoshBuddy uses `env_clear` so your current desktop theme doesn't "pollute" the preview. What you see is the pure, raw theme output.

⚠️ **Warning**: Icons in the preview might look broken if PoshBuddy is running in a terminal without a Nerd Font active, even if the font is installed on the system.

---
**Next Step**: [Troubleshooting](troubleshooting)
