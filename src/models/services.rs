use crate::services;

pub struct Services<
    SqlService: services::sql::SqliteService,
    EncService: services::encryption::EncryptionService,
> {
    pub encryption: EncService,
    pub sql: SqlService,
}
