//! Enhanced plugin installation module for PoshBuddy
//!
//! Provides utilities for PowerShell module management.

use std::io;
use std::process::Command;

/// Plugin installer
pub struct PluginInstaller;

impl PluginInstaller {
    /// Creates a new installer
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    /// Verifica si PowerShell está disponible
    #[allow(dead_code)]
    fn check_powershell_available() -> bool {
        Command::new("pwsh")
            .args(["-Command", "$PSVersionTable.PSVersion.Major"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or_else(|_| {
                // Intentar con powershell (versión antigua)
                Command::new("powershell")
                    .args(["-Command", "$PSVersionTable.PSVersion.Major"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            })
    }

    /// Checks if a module is already installed
    #[allow(dead_code)]
    fn check_module_installed(module_name: &str) -> Result<bool, io::Error> {
        let output = Command::new("pwsh")
            .env("POSHBUDDY_MODULE_NAME", module_name)
            .args([
                "-Command",
                "Get-Module -ListAvailable -Name $env:POSHBUDDY_MODULE_NAME | Select-Object -First 1",
            ])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(!stdout.trim().is_empty())
        } else {
            Err(io::Error::other("Failed to check module status"))
        }
    }

    /// Checks PowerShell execution policy
    #[allow(dead_code)]
    fn check_execution_policy() -> Result<String, io::Error> {
        let output = Command::new("pwsh")
            .args(["-Command", "Get-ExecutionPolicy -Scope CurrentUser"])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(io::Error::other("Failed to check execution policy"))
        }
    }

    /// Checks basic internet connectivity
    #[allow(dead_code)]
    fn check_internet_connectivity() -> bool {
        // Attempts to ping Google DNS as a simple test
        #[cfg(windows)]
        {
            Command::new("ping")
                .args(["-n", "1", "-w", "1000", "8.8.8.8"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
        #[cfg(not(windows))]
        {
            Command::new("ping")
                .args(["-c", "1", "-W", "1", "8.8.8.8"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
    }

    /// Uninstalls a module
    #[allow(dead_code)]
    pub async fn uninstall_module(&self, module_name: &str) -> Result<(), io::Error> {
        let output = tokio::process::Command::new("powershell")
            .env("POSHBUDDY_MODULE_NAME", module_name)
            .args([
                "-Command",
                "Uninstall-Module -Name $env:POSHBUDDY_MODULE_NAME -Scope CurrentUser -Force -Confirm:$false",
            ])
            .output()
            .await?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::other(format!("Failed to uninstall: {}", stderr)))
        }
    }

    /// Gets information about an installed module
    #[allow(dead_code)]
    pub fn get_module_info(&self, module_name: &str) -> Result<Option<ModuleInfo>, io::Error> {
        let output = Command::new("pwsh")
            .env("POSHBUDDY_MODULE_NAME", module_name)
            .args([
                "-Command",
                "Get-Module -ListAvailable -Name $env:POSHBUDDY_MODULE_NAME | Select-Object Name, Version, Description | Format-List",
            ])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                return Ok(None);
            }

            // Simple output parsing
            let mut name = None;
            let mut version = None;
            let mut description = None;

            for line in stdout.lines() {
                if line.starts_with("Name") {
                    name = line.split(':').nth(1).map(|s| s.trim().to_string());
                } else if line.starts_with("Version") {
                    version = line.split(':').nth(1).map(|s| s.trim().to_string());
                } else if line.starts_with("Description") {
                    description = line.split(':').nth(1).map(|s| s.trim().to_string());
                }
            }

            Ok(Some(ModuleInfo {
                name: name.unwrap_or_default(),
                version: version.unwrap_or_default(),
                description: description.unwrap_or_default(),
            }))
        } else {
            Err(io::Error::other("Failed to get module info"))
        }
    }
}

impl Default for PluginInstaller {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about an installed module
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Nota: Los tests que requieren PowerShell están marcados como #[ignore]
    // porque pueden no funcionar en todos los entornos

    #[test]
    #[ignore = "Requires PowerShell"]
    fn test_check_powershell_available() {
        assert!(PluginInstaller::check_powershell_available());
    }
}
