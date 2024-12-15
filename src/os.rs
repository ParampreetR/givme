use crate::models::enums::OperatingSystem;
use std::path::Path;

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
            OperatingSystem::Mac => Ok(!(Path::new(
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
