use std::{env, io::Write, path::Path};

use rpassword::read_password;

use crate::models::credentials::Credentials;

pub fn ask_user_for_value(key: &str) -> Result<Credentials, std::io::Error> {
    print!("Enter your '{}': ", key);
    std::io::stdout().flush()?;
    let password = read_password().unwrap().trim().to_string();
    print!("Any note for yourself: ");
    std::io::stdout().flush()?;
    let mut info = String::new();
    std::io::stdin().read_line(&mut info)?;
    info = info.trim().to_string();
    Ok(Credentials::new(key.to_string(), password, info))
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
