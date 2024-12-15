use crate::{error::ErrorDetails, services};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    fn delete_by_keyname(&self, keyname: String) -> Result<(), ErrorDetails> {
        let key = base64::encode(self.encryption_service.encrypt(keyname.clone())?);
        self.sql_service.del_from_sql(key)?;

        println!("'{}' deleted successfully", keyname);
        Ok(())
    }
}
