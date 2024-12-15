use std::io::Write;

use crate::models::{credentials::Credentials, enums::OperatingSystem};
use log::{debug, error};
use sqlite::{Connection, State};

pub trait SqliteService {
    fn new(connection: Connection) -> impl SqliteService {
        SqliteStruct { connection }
    }

    fn get_from_sql(&self, key: &str) -> Vec<Credentials>;
    fn save_to_sql(&self, cred: Credentials) -> Result<(), sqlite::Error>;
    fn del_from_sql(&self, key: String) -> Result<(), sqlite::Error>;

    fn already_exist_in_sql(&self, key: String) -> Result<bool, sqlite::Error>;
}

pub struct SqliteStruct {
    connection: Connection,
}

impl SqliteService for SqliteStruct {
    /// Retreive Data from Sqlite Database by querying given key
    fn get_from_sql(&self, key: &str) -> Vec<Credentials> {
        let mut statement = self
            .connection
            .prepare(format!("SELECT * FROM cred WHERE key = '{}'", key))
            .unwrap();
        let mut cred: Credentials;
        let mut cred_vec: Vec<Credentials> = Vec::new();

        while let State::Row = statement.next().unwrap() {
            cred = Credentials::new(
                statement.read::<String>(0).unwrap(),
                statement.read::<String>(1).unwrap(),
                statement.read::<String>(2).unwrap(),
            );
            cred_vec.push(cred);
        }

        cred_vec
    }

    /// Saves data to Sqlite database
    fn save_to_sql(&self, cred: Credentials) -> Result<(), sqlite::Error> {
        let mut option = String::new();

        let cred = cred.provide();

        let mut statement = self
            .connection
            .prepare("SELECT COUNT(*) FROM cred WHERE key = ?")
            .unwrap();

        let mut count: i64 = 0;

        // Bind the key to the statement and execute
        statement.bind(1, &*cred.0).unwrap();

        // Step through the result to get the count
        while let sqlite::State::Row = statement.next().unwrap() {
            count = statement.read::<i64>(0).unwrap(); // Read the first column (the count)
        }

        if count > 0 {
            println!("{}", count);
            println!("Record with key {} already exist", cred.0);
            print!("Do you want to overwrite? (y/n) ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut option).unwrap();
            if option.chars().next().is_some() {
                if option.to_lowercase().chars().next().unwrap() == 'y' {
                    self.connection
                        .execute(format!(
                            "UPDATE cred SET value = '{}' info = '{}' WHERE key = '{}'",
                            cred.1, cred.2, cred.0
                        ))
                        .unwrap();
                }
            }
        } else {
            self.connection
                .execute(format!(
                    "INSERT INTO cred VALUES ('{}', '{}', '{}')",
                    cred.0, cred.1, cred.2
                ))
                .unwrap();
        }
        Ok(())
    }

    /// Deletes data to Sqlite database
    fn del_from_sql(&self, key: String) -> Result<(), sqlite::Error> {
        self.connection
            .execute(format!("DELETE FROM cred WHERE key='{}'", key))
    }

    /// Checks if value already exist in Sqlite
    fn already_exist_in_sql(&self, key: String) -> Result<bool, sqlite::Error> {
        match self
            .connection
            .prepare(format!("SELECT value FROM cred WHERE key = '{}'", &key))
        {
            Ok(mut r) => {
                if r.next().unwrap() == sqlite::State::Row {
                    //                println!("Exist {:?}", a);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => Err(err),
        }
    }
}
