use std::{env, path::Path};

/// Check if our database file exist. If not, then
/// this is our first run.
pub fn is_first_run() -> Result<bool, String> {
    if env::var("DEBUG").is_ok() {
        Ok(!(Path::new("./cred_debug.db").exists()))
    } else {
        Ok(dirs::config_dir()
            .unwrap()
            .join(".\\givme\\cred.db")
            .exists())
    }
}
