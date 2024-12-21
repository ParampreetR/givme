use crate::services;

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    pub fn encrypt_file(
        &self,
        filepath: &str,
        destpath: &str,
    ) -> anyhow::Result<(), anyhow::Error> {
        match self.encryption_service.encrypt_file(filepath, destpath) {
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
