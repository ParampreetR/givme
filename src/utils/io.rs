use std::{env, io::Write, path::Path};

use rpassword::read_password;

use crate::models::credentials::Credentials;

pub fn ask_user_for_value(key: &str) -> anyhow::Result<Credentials, anyhow::Error> {
    let value_to_save = inquire::prompt_secret("Value: ")?;
    let note_to_save = inquire::prompt_text("Note (Optional): ")?;
    Ok(Credentials::new(key, &value_to_save, &note_to_save))
}

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
