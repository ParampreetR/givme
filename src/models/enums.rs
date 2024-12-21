use std::env::consts::OS;

pub enum OperatingSystem {
    Windows,
    Linux,
    Mac,
    Other,
}

/// Checks for current OS and user. Populate in provided
/// `GivMe` struct
///
/// Only supports Windows and Linux. Rest are included in
/// other OS.
pub fn get_os_and_username() -> Option<OperatingSystem> {
    match OS {
        "linux" => Some(OperatingSystem::Linux),
        "windows" => Some(OperatingSystem::Windows),
        "macos" => Some(OperatingSystem::Mac),
        _ => Some(OperatingSystem::Other),
    }
}
