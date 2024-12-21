use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Debug, Insertable)]
#[diesel(table_name = crate::utils::schema::credentials)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Credentials {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub info: Option<String>,
}

impl Credentials {
    /// Create new instance of `Credentials`
    pub fn new(key: &str, value: &str, info: &str) -> Self {
        if info.is_empty() {
            Credentials {
                id: 0,
                key: key.to_string(),
                value: value.to_string(),
                info: None,
            }
        } else {
            Credentials {
                id: 0,
                key: key.to_string(),
                value: value.to_string(),
                info: Some(info.to_string()),
            }
        }
    }

    /// Returns tuple for data in current struct by
    /// dealing with info which is `Option<String>
    pub fn provide(&self) -> (String, String, String) {
        if let Some(info) = self.info.clone() {
            (self.key.clone(), self.value.clone(), info.clone())
        } else {
            (self.key.clone(), self.value.clone(), String::new())
        }
    }
}
