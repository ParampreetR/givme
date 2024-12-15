use std::{env, fs, io::Write, os};

use log::{debug, error};
use rpassword::read_password;
use sqlite::Connection;
use whoami::username;

use crate::{
    enums, models, services,
    utils::{self, helpers},
};

fn init_services() -> models::dependencies::Dependencies<
    impl services::encryption::EncryptionService,
    impl services::sql::SqliteService,
> {
    utils::logger::init_logger();
    let sql_connection = get_sql_con().expect("Unable to open Sql Connection");

    let sql_service =
        <services::sql::SqliteStruct as services::sql::SqliteService>::new(sql_connection);
    let enc_service =
        <services::encryption::Encryption as services::encryption::EncryptionService>::new(
            "".to_string(),
            "".to_string(),
        );

    let services = models::services::Services {
        encryption: enc_service,
        sql: sql_service,
    };

    let master_pass = ask_master_pass();
    let secret_key = extract_secret_key(&services).expect("Unable to extract secret key");

    models::dependencies::Dependencies {
        key: secret_key,
        password: master_pass,
        os: enums::get_os_and_username().expect("OS not supported"),
        username: username(),
        services,
    }
}

fn extract_secret_key(
    services: &models::services::Services<
        impl services::encryption::EncryptionService,
        impl services::sql::SqliteService,
    >,
) -> Result<String, String> {
    let encrypted_key = base64::decode(&services.sql.get_from_sql("secret_key")[0].value).unwrap();
    let decrypted_pass = services.encryption.decrypt(&encrypted_key);

    if decrypted_pass.is_err() {
        eprintln!("Error: Invalid Password");
        return Err(decrypted_pass.unwrap_err());
    }
    let decrypted_pass = decrypted_pass.unwrap();

    debug!("Decrypted pass: {}", decrypted_pass);
    return Ok((&decrypted_pass)[4..32].to_string());
}

/// SetUp Sqlite Database. Like create new Database file and Create
/// new table in newly created Database file
fn setup_sql(sql_connection: Connection) {
    match sql_connection.execute("CREATE TABLE cred (key TEXT, value TEXT, info TEXT)") {
        Ok(_) => {}
        Err(e) => {
            error!("{e}");
        }
    };
}

/// Open new Sql Connection to file and populate it in
/// `GivMe` Struct.
///
/// Behaviour changes when DEBUG is enabled
fn get_sql_con() -> Option<Connection> {
    let config_dir = dirs::config_dir().expect("Unable to get config dir");
    if env::var("DEBUG").is_ok() {
        Some(Connection::open("./cred_debug.db").unwrap())
    } else {
        fs::create_dir_all(config_dir.join("./givme").to_str().unwrap());
        Some(Connection::open(config_dir.join("./givme/cred.db")).unwrap())
    }
}

pub fn ask_master_pass() -> String {
    eprint!("Enter your Master Key: ");
    std::io::stdout().flush().unwrap();
    let user_entered_pass = read_password().unwrap().trim().to_string();
    let proper_length_password = helpers::adjust_password_length(&user_entered_pass, 24);
    return proper_length_password;
}
