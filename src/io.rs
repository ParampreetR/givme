use crate::encryption::decrypt;
use crate::sql::get_from_sql;
use crate::structs::{Credentials, GivMe};
use crate::utils::*;
use clap::{App, Arg};
use lazy_static::lazy_static;
use rpassword::read_password;
use std::{env, io::Write};

lazy_static! {
    pub static ref DEBUG: bool = env::var("GIVME_DEBUG").is_ok();
}

/// Authenticate User by verifying user entered password
/// with decrypted password.
///
/// This function also does some random string decryption as dual
/// security measure.
pub fn ask_pass_and_extract_key(handle: &mut GivMe) -> Result<bool, bool> {
    eprint!("Enter your Master Key: ");
    std::io::stdout().flush().unwrap();
    let user_entered_pass = read_password().unwrap().trim().to_string();
    let proper_length_password = adjust_password_length(&user_entered_pass, 24);
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

/// Ask for value and info to construct `Credentials`
///
/// `info` is optional and is stored as Option<String>.
/// Unlike other values like `key` or `value`.
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

/// Print debug messages
pub fn debug(msg: &str) {
    if *DEBUG {
        eprintln!("[Debug]: {}", msg);
    }
}

pub fn parse_args() -> clap::App<'static, 'static> {
    App::new("givme")
        .about("A Simple and Secure Password Manager")
        .arg(
            Arg::with_name("store")
                .short("s")
                .long("store")
                .value_name("KEY")
                .help("Asks value for key to store safely")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .help("Key to query")
                .required(false)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("raw")
                .short("r")
                .required(false)
                .takes_value(false)
                .help("Outputs only value for a key"),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .required(false)
                .value_name("KEY")
                .long("delete")
                .takes_value(true)
                .help("Deletes key value pair of given key"),
        )
        .arg(
            Arg::with_name("encrypt-file")
                .required(false)
                .value_name("SOURCE and DESTINATION FILE")
                .long("enc-file")
                .takes_value(true)
                .number_of_values(2)
                .help("Encrypts specified file"),
        )
        .arg(
            Arg::with_name("decrypt-file")
                .required(false)
                .value_name("SOURCE and DESTINATION FILE")
                .long("dec-file")
                .takes_value(true)
                .number_of_values(2)
                .help("Decrypts specified encrypted file"),
        )
        .arg(
            Arg::with_name("get-secret-key")
                .required(false)
                .long("get-secret-key")
                .takes_value(false)
                .help("Retrieve secret key from database"),
        )
        .arg(
            Arg::with_name("set-secret-key")
                .required(false)
                .value_name("secretkey")
                .long("set-secret-key")
                .takes_value(true)
                .number_of_values(1)
                .help("Sets the secret key of one encryption standard"),
        )
    // .subcommand(
    //     SubCommand::with_name("test")
    //         .about("controls testing features")
    //         .version("1.3")
    //         .author("Someone E. <someone_else@other.com>")
    //         .arg(
    //             Arg::with_name("debug")
    //                 .short("d")
    //                 .help("print debug information verbosely"),
    //         ),
    // )
}
