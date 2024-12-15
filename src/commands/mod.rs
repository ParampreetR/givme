use crate::{models, services};

mod decrypt_file;
mod decrypt_value;
mod encrypt_file;
mod encrypt_value;
mod update_secret_key;

trait Commands {}

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
    fn new(srvs: models::services::Services<SqlSrv, EncSrv>) -> Self {
        Self {
            sql_service: srvs.sql,
            encryption_service: srvs.encryption,
        }
    }
}
