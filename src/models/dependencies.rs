use crate::services;

use super::{enums::OperatingSystem, services::Services};

pub struct Dependencies<
    EncService: services::encryption::EncryptionService,
    SqlService: services::sql::SqliteService,
> {
    pub key: String, /* Key Size must be 32 Bytes long. Repeat the sequence when storing in struct. */
    pub password: String,
    pub os: OperatingSystem,
    pub username: String,
    pub services: Services<SqlService, EncService>,
}
