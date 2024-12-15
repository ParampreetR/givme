use std::io::{self, Write};

use crate::{error::ErrorDetails, services};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    fn decrypt_file(
        &self,
        encrypted_filepath: String,
        destpath: String,
    ) -> Result<(), ErrorDetails> {
        self.encryption_service
            .decrypt_file(encrypted_filepath.clone(), destpath.clone())?;

        println!(
            "{} Decrypted Successfully to {}",
            encrypted_filepath, destpath
        );

        Ok(())
    }
}
