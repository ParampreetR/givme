use crate::{models, services};

mod decrypt_file;
mod delete_value;
mod encrypt_file;
mod get_value;
mod init_tool;
mod save_value;
mod secret_key;

struct CommandStruct<
    SqlSrv: services::sql::SqliteService,
    EncSrv: services::encryption::EncryptionService,
> {
    sql_service: SqlSrv,
    encryption_service: EncSrv,
}

impl<SqlSrv: services::sql::SqliteService, EncSrv: services::encryption::EncryptionService>
    CommandStruct<SqlSrv, EncSrv>
{
    pub fn new(srvs: models::services::Services<SqlSrv, EncSrv>) -> Self {
        Self {
            sql_service: srvs.sql,
            encryption_service: srvs.encryption,
        }
    }
}
