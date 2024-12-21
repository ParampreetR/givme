use std::io::Error;

use crate::{services, utils::io::ask_user_for_value};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    pub fn save_encrypt_value(&mut self, key: String) -> anyhow::Result<String, anyhow::Error> {
        let mut cred = ask_user_for_value(&key).unwrap();

        match self.sql_service.already_exist_in_sql(&cred.key) {
            Ok(exist) => {
                if exist {
                    return Err(anyhow::anyhow!("key with the same name already exist"));
                } else {
                    cred.value =
                        base64::encode(self.encryption_service.encrypt(cred.value).unwrap());
                    if let Some(info) = cred.info {
                        cred.info = Some(base64::encode(
                            self.encryption_service.encrypt(info).unwrap(),
                        ));
                    }

                    self.sql_service.save_to_sql(cred)?;
                    return Ok(format!("Saved Successfully!"));
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
