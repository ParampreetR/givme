use crate::services;

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    fn delete_by_keyname(&mut self, keyname: String) -> anyhow::Result<(), anyhow::Error> {
        let key = base64::encode(self.encryption_service.encrypt(keyname.clone())?);
        self.sql_service.del_from_sql(key)?;

        println!("'{}' deleted successfully", keyname);
        Ok(())
    }
}
