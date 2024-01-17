use crate::encryption::encrypt;
use crate::io::debug;
use crate::sql::{save_to_sql, setup_sql};
use crate::structs::{Credentials, GivMe};
use rand::random;
use rpassword::read_password;
use std::io::Write;

pub const BANNER: &str =
    "████████████████████████████████████████████████████████████████████████████████
█░░░░░░░░░░░░░░█░░░░░░░░░░█░░░░░░██░░░░░░█░░░░░░██████████░░░░░░█░░░░░░░░░░░░░░█
█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░░░░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░▄▀░░░░░░░░░░█░░░░▄▀░░░░█░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░███████████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░▄▀░░░░░░▄▀░░█░░▄▀░░█████████
█░░▄▀░░███████████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░██░░░░░░███░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░▄▀░░██░░▄▀░░███░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░░░░░██░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░██░░▄▀░░███░░▄▀░░███░░▄▀▄▀░░▄▀▄▀░░█░░▄▀░░██████████░░▄▀░░█░░▄▀░░█████████
█░░▄▀░░░░░░▄▀░░█░░░░▄▀░░░░█░░░░▄▀▄▀▄▀░░░░█░░▄▀░░██████████░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀░░███░░░░▄▀░░░░███░░▄▀░░██████████░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░░░░░░░░░░░░░█░░░░░░░░░░█████░░░░░░█████░░░░░░██████████░░░░░░█░░░░░░░░░░░░░░█
████████████████████████████████████████████████████████████████████████████████";

/// Returns random sequence of characters with provided length
///
/// Bydefault it avoids any "'" in randomly generated String.
/// This is done to avoid any errors while giving this String
/// in SQL Command.
fn rand_string(size: usize) -> String {
    loop {
        let random_string: String = (0..size)
            .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
            .collect();
        if !random_string.contains("'") {
            return random_string;
        }
    }
}

/// Ask user to set master password. Encrypt it and save it to Sqlite.
pub fn run_setup(handle: &mut GivMe) -> Result<bool, bool> {
    let common_passes = vec![
        "123456",
        "123456789",
        "picture1",
        "password",
        "12345678",
        "111111",
        "123123",
        "12345",
        "1234567890",
        "senha",
    ];
    let mut warn_given: bool = false;
    let mut password;
    let mut confirm_password;
    println!("{}", BANNER);

    loop {
        print!("Set your Master Key: ");
        std::io::stdout().flush().unwrap();
        password = read_password().unwrap().trim().to_string();
        debug("Adjusting password size");
        if common_passes.contains(&password.as_str()) {
            if warn_given {
                eprintln!("This is very common password. Try something else.");
                continue;
            } else {
                eprintln!("Seriously??");
                warn_given = true;
            }
        } else {
            print!("Confirm your Master Key: ");
            std::io::stdout().flush().unwrap();
            confirm_password = read_password().unwrap().trim().to_string();
            if confirm_password != password {
                eprintln!("Unmatched Master Key. Try Again...\n");
            } else {
                break;
            }
        }
    }
    password = adjust_password_length(&password, 24);
    debug("Encrypting password to store in data");
    setup_sql(handle);
    handle.password = Some(password.clone());

    let encrypted = encrypt(
        format!("{}", rand_string(32)),
        &mut GivMe {
            key: Some(password.clone()),
            sql_con: None,
            password: Some(password.clone()),
            os: None,
            username: None,
        },
    )
    .unwrap();

    let encrypted_final_key = base64::encode(encrypted);
    debug("Adding to database");
    save_to_sql(
        Credentials::new(
            String::from("secret_key"),
            encrypted_final_key,
            String::new(),
        ),
        handle,
    )
    .unwrap();
    //println!("{}", password);
    Ok(true)
}

/// As a security measure and shortcommings of encryption algorithms.
/// We need to adjust password to some specified length. This is done
/// by repeating password couple of times.
pub fn adjust_password_length(input_password: &str, length: usize) -> String {
    let mut proper_length_password = String::new();
    if input_password.len() > length {
        eprintln!("Slicing password to first length digits...");
        proper_length_password = input_password[..length].to_string();
    } else {
        loop {
            for single_char in input_password.chars() {
                proper_length_password.push(single_char);
                if proper_length_password.len() == length {
                    break;
                }
            }
            if proper_length_password.len() == length {
                break;
            }
        }
    }
    proper_length_password
}
