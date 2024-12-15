use std::io::Write;

use rpassword::read_password;

use crate::credentials::Credentials;

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

pub fn ask_master_pass(handle: &mut GivMe) -> Result<bool, bool> {
    eprint!("Enter your Master Key: ");
    std::io::stdout().flush().unwrap();
    let user_entered_pass = read_password().unwrap().trim().to_string();
    let proper_length_password = helpers::adjust_password_length(&user_entered_pass, 24);
    let encrypted_key = base64::decode(&get_from_sql("secret_key", handle)[0].value).unwrap();
    let decrypted_pass = decrypt(
        &encrypted_key,
        &GivMe {
            key: Some(proper_length_password.clone()),
            sql_con: None,
            password: Some(proper_length_password.clone()),
            os: None,
            username: None,
        },
    );

    if decrypted_pass.is_err() {
        eprintln!("Error: Invalid Password");
        std::process::exit(1);
    }
    let decrypted_pass = decrypted_pass.unwrap();

    debug(format!("Decrypted pass: {}", decrypted_pass).as_str());
    handle.key = Some((&decrypted_pass)[4..32].to_string());
    handle.password = Some(proper_length_password);
    Ok(true)
}
