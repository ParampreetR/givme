use crate::{models::credentials::Credentials, utils};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};
use inquire::error::InquireResult;
use utils::schema::credentials::dsl::*;

pub trait SqliteService {
    fn new(connection: SqliteConnection) -> impl SqliteService {
        SqliteStruct { connection }
    }

    fn get_from_sql(&mut self, key: &str) -> Result<Vec<Credentials>, anyhow::Error>;
    fn save_to_sql(&mut self, cred: Credentials) -> Result<(), anyhow::Error>;
    fn del_from_sql(&mut self, key: String) -> Result<(), anyhow::Error>;
    fn already_exist_in_sql(&mut self, key: &str) -> Result<bool, anyhow::Error>;
}

pub struct SqliteStruct {
    connection: SqliteConnection,
}

impl SqliteService for SqliteStruct {
    /// Retreive Data from Sqlite Database by querying given key
    fn get_from_sql(&mut self, user_key: &str) -> Result<Vec<Credentials>, anyhow::Error> {
        let credentails_result: Vec<Credentials> =
            credentials.filter(key.eq(key)).load(&mut self.connection)?;

        let mut credentials_vector = vec![];

        for credential in credentails_result {
            let cred_struct = Credentials::new(
                &credential.key,
                &credential.value,
                &credential.info.unwrap_or_else(|| return "".to_string()),
            );

            credentials_vector.push(cred_struct);
        }

        Ok(credentials_vector)
    }

    /// Saves data to Sqlite database
    fn save_to_sql(&mut self, cred: Credentials) -> Result<(), anyhow::Error> {
        let credentials_from_table = self.get_from_sql(&cred.key)?;

        if credentials_from_table.len() > 0 {
            println!("{:?}", credentials_from_table);
            match inquire::prompt_confirmation("Record already exist. Overwrite? ") {
                InquireResult::Ok(true) => {
                    diesel::update(credentials)
                        .filter(key.eq(&cred.key))
                        .set((key.eq(&cred.key), value.eq(cred.value), info.eq(cred.info)))
                        .execute(&mut self.connection);
                }
                InquireResult::Ok(false) => {}
                InquireResult::Err(err) => return Err(err.into()),
            }
        } else {
            diesel::insert_into(credentials)
                .values(&cred)
                .execute(&mut self.connection);
        }
        Ok(())
    }

    /// Deletes data to Sqlite database
    fn del_from_sql(&mut self, user_key: String) -> Result<(), anyhow::Error> {
        diesel::delete(credentials)
            .filter(key.eq(user_key))
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// Checks if value already exist in Sqlite
    fn already_exist_in_sql(&mut self, user_key: &str) -> Result<bool, anyhow::Error> {
        return Ok(self.get_from_sql(user_key)?.len() > 0);
    }
}
