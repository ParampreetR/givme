use sqlite::Connection;

use super::enums::OperatingSystem;

pub struct GivMe {
    pub key: Option<String>, /* Key Size must be 32 Bytes long. Repeat the sequence when storing in struct. */
    pub password: Option<String>,
    pub sql_con: Option<Connection>,
    pub os: Option<OperatingSystem>,
    pub username: Option<String>,
}

impl GivMe {
    pub fn new() -> Self {
        GivMe {
            key: None,
            password: None,
            sql_con: None,
            username: None,
            os: None,
        }
    }
}
