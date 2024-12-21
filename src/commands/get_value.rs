use std::io::{self, Write};

use crate::{models::credentials::Credentials, services};

use super::CommandStruct;

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    pub fn get_decrypt_value(&mut self, key: &str) -> anyhow::Result<String, anyhow::Error> {
        let original_key =
            base64::encode(self.encryption_service.encrypt(key.to_string()).unwrap());
        let mut creds: Vec<Credentials> = self.sql_service.get_from_sql(&original_key)?;
        if creds.is_empty() {
            return Err(anyhow::anyhow!(
                "Unable to retrieve value, record may not exist",
            ));
        } else {
            creds[0].key = key.to_string();
            creds[0].value = self
                .encryption_service
                .decrypt(&base64::decode(creds[0].value.clone()).unwrap())
                .unwrap()
                .to_string();
            if let Some(info) = &creds[0].info {
                creds[0].info = Some(
                    self.encryption_service
                        .decrypt(&base64::decode(info).unwrap())
                        .unwrap()
                        .to_string(),
                );
                print!("{:?}", creds[0].clone());
            } else {
                print!("{:?}", creds[0].clone());
            }
            io::stdout().flush().unwrap();
            Ok(format!("{:?}", creds[0].clone()))
        }
    }
}
