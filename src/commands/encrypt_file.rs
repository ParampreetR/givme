use std::io::Error;

use crate::{
    enums::ErrorType, error::ErrorDetails, models, save_credentials, services,
    utils::io::ask_user_for_value,
};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    fn encrypt_file(&self, filepath: String, destpath: String) -> Result<(), ErrorDetails> {
        match self
            .encryption_service
            .encrypt_file(filepath.clone(), destpath.clone())
        {
            Ok(_) => {
                println!("{} Encrypted Successfully to {}", filepath, destpath);
            }
            Err(err) => {
                eprintln!("-- Error in Encryption of file '{}'", filepath);
                eprintln!("{:?}", err);
            }
        };
        Ok(())
    }
}
