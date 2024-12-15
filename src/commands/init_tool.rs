use std::io::Write;

use log::debug;
use rpassword::read_password;

use crate::{
    models::{credentials::Credentials, error::ErrorDetails},
    services,
    utils::helpers::{adjust_password_length, rand_string, BANNER},
};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    fn init_setup(&self, key: String) -> Result<(), ErrorDetails> {
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
            debug!("Adjusting password size");
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
        debug!("Encrypting password to store in data");
        // setup_sql();

        let encrypted = self
            .encryption_service
            .encrypt(format!("{}", rand_string(32)))
            .unwrap();

        let encrypted_final_key = base64::encode(encrypted);
        debug!("Adding to database");
        self.sql_service
            .save_to_sql(Credentials::new(
                String::from("secret_key"),
                encrypted_final_key,
                String::new(),
            ))
            .unwrap();
        //println!("{}", password);
        Ok(())
    }
}
