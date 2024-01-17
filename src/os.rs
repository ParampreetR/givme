use crate::io::{debug, DEBUG};
use crate::structs::{GivMe, OperatingSystem};
use std::env::consts::OS;
use std::path::Path;

/// Checks for current OS and user. Populate in provided
/// `GivMe` struct
///
/// Only supports Windows and Linux. Rest are included in
/// other OS.
pub fn get_os_and_username(handle: &mut GivMe) {
    debug(OS);
    match OS {
        "linux" => handle.os = Some(OperatingSystem::Linux),
        "windows" => handle.os = Some(OperatingSystem::Windows),
        _ => handle.os = Some(OperatingSystem::Other),
    };
    handle.username = Some(whoami::username());
}

/// Check if our database file exist. If not, then
/// this is our first run.
pub fn is_first_run(handle: &GivMe) -> Result<bool, String> {
    if *DEBUG {
        Ok(!(Path::new("./cred_debug.db").exists()))
    } else {
        match handle.os.as_ref().unwrap() {
            OperatingSystem::Linux => Ok(!(Path::new(
                format!(
                    "{}/.config/givme/cred.db",
                    home::home_dir()
                        .expect("Consider settings for home dir")
                        .display()
                )
                .as_str(),
            )
            .exists())),
            OperatingSystem::Windows => Ok(!(Path::new(
                format!(
                    "C:\\Users\\{}\\givme\\cred.db",
                    handle.username.as_ref().unwrap()
                )
                .as_str(),
            )
            .exists())),
            /* Maybe working on some other platforms like MacOS but still not sure */
            OperatingSystem::Other => Err(String::from("Unsupported Platform")),
        }
    }
}
