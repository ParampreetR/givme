use std::{env, error::Error, fs, io::Write, os};

use diesel::{sqlite::Sqlite, Connection, ConnectionResult, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use inquire::Password;
use log::{debug, error};
use rpassword::read_password;
use whoami::username;

use crate::{
    models, services,
    utils::{self, helpers},
};

pub fn init_services() -> anyhow::Result<
    models::dependencies::Dependencies<
        impl services::encryption::EncryptionService,
        impl services::sql::SqliteService,
    >,
    anyhow::Error,
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

    let mut services = models::services::Services {
        encryption: enc_service,
        sql: sql_service,
    };

    let master_pass = ask_master_pass();
    let secret_key = extract_secret_key(&mut services)?;

    Ok(models::dependencies::Dependencies {
        key: secret_key,
        password: master_pass?,
        os: models::enums::get_os_and_username().expect("OS not supported"),
        username: username(),
        services,
    })
}

fn extract_secret_key(
    services: &mut models::services::Services<
        impl services::sql::SqliteService,
        impl services::encryption::EncryptionService,
    >,
) -> anyhow::Result<String, anyhow::Error> {
    let encrypted_key = base64::decode(
        services.sql.get_from_sql("secret_key").unwrap()[0]
            .value
            .clone(),
    )
    .unwrap();
    let decrypted_pass = services.encryption.decrypt(&encrypted_key)?;

    debug!("Decrypted pass: {}", decrypted_pass);
    return Ok((&decrypted_pass)[4..32].to_string());
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// SetUp Sqlite Database. Like create new Database file and Create
/// new table in newly created Database file
fn migrate_changes(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

/// Open new Sql Connection to file and populate it in
/// `GivMe` Struct.
///
/// Behaviour changes when DEBUG is enabled
fn get_sql_con() -> ConnectionResult<SqliteConnection> {
    let config_dir = dirs::config_dir().expect("Unable to get config dir");
    if env::var("DEBUG").is_ok() {
        SqliteConnection::establish("./cred_debug.db")
    } else {
        fs::create_dir_all(
            config_dir
                .join("./givme")
                .to_str()
                .expect("unable to get config dir path"),
        );
        SqliteConnection::establish(
            config_dir
                .join("./givme/cred.db")
                .to_str()
                .expect("unable to get config dir path"),
        )
    }
}

pub fn ask_master_pass() -> anyhow::Result<String, anyhow::Error> {
    let master_pass = Password::new("Enter your Master Key: ").prompt()?;
    let proper_length_password = helpers::adjust_password_length(&master_pass, 24);
    return Ok(proper_length_password);
}
