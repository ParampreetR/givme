use std::io::Write;

use crate::models::{credentials::Credentials, enums::OperatingSystem, givme::GivMe};
use log::{debug, error};
use sqlite::{Connection, State};

trait SqliteService {
    fn new(connection: Connection) -> impl SqliteService {
        SqliteStruct { connection }
    }

    fn get_sql_con(&self, handle: &mut GivMe);
    fn get_from_sql(&self, key: &str) -> Vec<Credentials>;
    fn save_to_sql(&self, cred: Credentials) -> Result<(), sqlite::Error>;
    fn del_from_sql(&self, key: String) -> Result<(), sqlite::Error>;
    fn setup_sql(&self, handle: &mut GivMe);
    fn already_exist_in_sql(&self, key: String) -> Result<bool, sqlite::Error>;
}

struct SqliteStruct {
    connection: Connection,
}

impl SqliteService for SqliteStruct {
    /// Open new Sql Connection to file and populate it in
    /// `GivMe` Struct.
    ///
    /// Behaviour changes when DEBUG is enabled
    fn get_sql_con(&self, handle: &mut GivMe) {
        if *DEBUG {
            handle.sql_con = Some(Connection::open("./cred_debug.db").unwrap())
        } else {
            handle.sql_con = match handle.os.as_ref().unwrap() {
                OperatingSystem::Linux => {
                    std::fs::create_dir_all(
                        format!(
                            "{}/.config/givme",
                            home::home_dir()
                                .expect("Consider setting for home dir")
                                .display()
                        )
                        .as_str(),
                    )
                    .unwrap();
                    Some(
                        Connection::open(
                            format!(
                                "{}/.config/givme/cred.db",
                                home::home_dir()
                                    .expect("Consider setting for home dir")
                                    .display()
                            )
                            .as_str(),
                        )
                        .unwrap(),
                    )
                }
                OperatingSystem::Mac => {
                    std::fs::create_dir_all(
                        format!(
                            "{}/.config/givme",
                            home::home_dir()
                                .expect("Consider setting for home dir")
                                .display()
                        )
                        .as_str(),
                    )
                    .unwrap();
                    Some(
                        Connection::open(
                            format!(
                                "{}/.config/givme/cred.db",
                                home::home_dir()
                                    .expect("Consider setting for home dir")
                                    .display()
                            )
                            .as_str(),
                        )
                        .unwrap(),
                    )
                }
                OperatingSystem::Windows => {
                    std::fs::create_dir_all(
                        format!("C:\\Users\\{}\\givme", handle.username.as_ref().unwrap()).as_str(),
                    )
                    .unwrap();
                    Some(
                        Connection::open(
                            format!(
                                "C:\\Users\\{}\\givme\\cred.db",
                                handle.username.as_ref().unwrap()
                            )
                            .as_str(),
                        )
                        .unwrap(),
                    )
                }
                OperatingSystem::Other => None,
            };
        }
    }

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

    /// SetUp Sqlite Database. Like create new Database file and Create
    /// new table in newly created Database file
    fn setup_sql(&self, handle: &mut GivMe) {
        match handle
            .sql_con
            .as_ref()
            .unwrap()
            .execute("CREATE TABLE cred (key TEXT, value TEXT, info TEXT)")
        {
            Ok(_) => {}
            Err(e) => {
                error!("{e}");
            }
        };
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
