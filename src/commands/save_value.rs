use std::io::Error;

use crate::{
    enums::ErrorType, error::ErrorDetails, models, save_credentials, services,
    utils::io::ask_user_for_value,
};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    fn save_encrypt_value(&self, key: String) -> Result<(), ErrorDetails> {
        let mut cred = ask_user_for_value(&key).unwrap();

        match self.sql_service.already_exist_in_sql(&cred.key) {
            Ok(exist) => {
                if exist {
                    return Err(ErrorDetails::new(
                        None,
                        Some("Key already exist in SQLite".to_string()),
                        ErrorType::Sqlite,
                    ));
                } else {
                    cred.value =
                        base64::encode(self.encryption_service.encrypt(cred.value).unwrap());
                    if let Some(info) = cred.info {
                        cred.info = Some(base64::encode(
                            self.encryption_service.encrypt(info).unwrap(),
                        ));
                    }

                    self.sql_service.save_to_sql(cred)?;
                    return Ok(());
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
