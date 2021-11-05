use lazy_static::lazy_static;
use nettle::cipher::{Cipher, Des3, Twofish};
use rand::random;
use rpassword::read_password;
use sqlite::{Connection, State};
use std::{env, env::consts::OS, io::Write, path::Path};

lazy_static! {
    static ref DEBUG: bool = env::var("GIVME_DEBUG").is_ok();
}

/// Returns random sequence of characters with provided length
///
/// Bydefault it avoids any "'" in randomly generated String.
/// This is done to avoid any errors while giving this String
/// in SQL Command.
fn rand_string(size: usize) -> String {
    loop {
        let random_string: String = (0..size)
            .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
            .collect();
        if !random_string.contains("'") {
            return random_string;
        }
    }
}

/// Checks for current OS and user. Populate in provided
/// `GivMe` struct
///
/// Only supports Windows and Linux. Rest are included in
/// other OS.
pub fn get_os_and_username(handle: &mut GivMe) {
    debug(OS);
    match OS {
        "linux" => handle.os = Some(OperatingSystem::Linux),
        "windows" => handle.os = Some(OperatingSystem::Windows),
        _ => handle.os = Some(OperatingSystem::Other),
    };
    handle.username = Some(whoami::username());
}

/// Used as a wrapper for data from/to Database
#[derive(Clone, Debug)]
pub struct Credentials {
    pub key: String,
    pub value: String,
    pub info: Option<String>,
}

impl Credentials {
    /// Create new instance of `Credentials`
    pub fn new(key: String, value: String, info: String) -> Self {
        if info.is_empty() {
            Credentials {
                key: key,
                value: value,
                info: None,
            }
        } else {
            Credentials {
                key: key,
                value: value,
                info: Some(info),
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

/// Print debug messages
fn debug(msg: &str) {
    if *DEBUG {
        eprintln!("[Debug]: {}", msg);
    }
}

/// Check if our database file exist. If not, then
/// this is our first run.
pub fn is_first_run(handle: &GivMe) -> Result<bool, String> {
    if *DEBUG {
        Ok(!(Path::new("./cred_debug.db").exists()))
    } else {
        match handle.os.as_ref().unwrap() {
            OperatingSystem::Linux => Ok(!(Path::new(
                format!(
                    "{}/.config/givme/cred.db",
                    home::home_dir()
                        .expect("Consider setting for home dir")
                        .display()
                )
                .as_str(),
            )
            .exists())),
            OperatingSystem::Windows => Ok(!(Path::new(
                format!(
                    "C:\\Users\\{}\\givme\\cred.db",
                    handle.username.as_ref().unwrap()
                )
                .as_str(),
            )
            .exists())),
            /* Maybe working on some other platforms like MacOS but still not sure */
            OperatingSystem::Other => Err(String::from("Unsupported Platform")),
        }
    }
}

/// Ask for value and info to construct `Credentials`
///
/// `info` is optional and is stored as Option<String>.
/// Unlike other values like `key` or `value`.
pub fn ask_user_for_value(key: &str) -> Result<Credentials, std::io::Error> {
    print!("Enter your '{}': ", key);
    std::io::stdout().flush()?;
    let password = read_password().unwrap().trim().to_string();
    print!("Any note for yourself: ");
    std::io::stdout().flush()?;
    let mut info = String::new();
    std::io::stdin().read_line(&mut info)?;
    info = info.trim().to_string();
    Ok(Credentials::new(key.to_string(), password, info))
}

/// Open new Sql Connection to file and populate it in
/// `GivMe` Struct.
///
/// Behaviour changes when DEBUG is enabled
pub fn get_sql_con(handle: &mut GivMe) {
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
pub fn get_from_sql(key: &str, handle: &GivMe) -> Vec<Credentials> {
    if handle.sql_con.is_none() {
        eprintln!("Sql connection was not initialized when getting query data from sql");
        std::process::exit(1);
    }

    let mut statement = handle
        .sql_con
        .as_ref()
        .unwrap()
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
pub fn save_to_sql(cred: Credentials, handle: &mut GivMe) -> Result<(), sqlite::Error> {
    if handle.sql_con.is_none() {
        eprintln!("Sql connection was not initialized when saving data from sql");
        std::process::exit(1);
    }

    let cred = cred.provide();

    handle.sql_con.as_ref().unwrap().execute(format!(
        "INSERT INTO cred VALUES ('{}', '{}', '{}')",
        cred.0, cred.1, cred.2
    ))
}

/// SetUp Sqlite Database. Like create new Database file and Create
/// new table in newly created Database file
pub fn setup_sql(handle: &mut GivMe) {
    get_sql_con(handle);
    match handle
        .sql_con
        .as_ref()
        .unwrap()
        .execute("CREATE TABLE cred (key TEXT, value TEXT, info TEXT)")
    {
        Ok(_) => {}
        Err(e) => {
            debug(format!("{:?}", e).as_str());
        }
    };
}

/// Ask user to set master password. Encrypt it and save it to Sqlite.
pub fn run_setup(handle: &mut GivMe) -> Result<bool, bool> {
    let common_passes = vec![
        "123456",
        "123456789",
        "picture1",
        "password",
        "12345678",
        "111111",
        "123123",
        "12345",
        "1234567890",
        "senha",
    ];
    let mut warn_given: bool = false;
    let mut password;
    let mut confirm_password;
    println!(
        "
████████████████████████████████████████████████████████████████████████████████
█░░░░░░░░░░░░░░█░░░░░░░░░░█░░░░░░██░░░░░░█░░░░░░██████████░░░░░░█░░░░░░░░░░░░░░█
█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░░░░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░▄▀░░░░░░░░░░█░░░░▄▀░░░░█░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░███████████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░▄▀░░░░░░▄▀░░█░░▄▀░░█████████
█░░▄▀░░███████████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░██░░░░░░███░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░▄▀░░██░░▄▀░░███░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░░░░░██░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░██░░▄▀░░███░░▄▀░░███░░▄▀▄▀░░▄▀▄▀░░█░░▄▀░░██████████░░▄▀░░█░░▄▀░░█████████
█░░▄▀░░░░░░▄▀░░█░░░░▄▀░░░░█░░░░▄▀▄▀▄▀░░░░█░░▄▀░░██████████░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀░░███░░░░▄▀░░░░███░░▄▀░░██████████░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░░░░░░░░░░░░░█░░░░░░░░░░█████░░░░░░█████░░░░░░██████████░░░░░░█░░░░░░░░░░░░░░█
████████████████████████████████████████████████████████████████████████████████
    \n"
    );

    loop {
        print!("Set your Master Key: ");
        std::io::stdout().flush().unwrap();
        password = read_password().unwrap().trim().to_string();
        debug("Adjusting password size");
        if common_passes.contains(&password.as_str()) {
            if warn_given {
                eprintln!("This is very common password. Try something else.");
                continue;
            } else {
                eprintln!("Seriously??");
                warn_given = true;
            }
        } else {
            print!("Confirm your Master Key: ");
            std::io::stdout().flush().unwrap();
            confirm_password = read_password().unwrap().trim().to_string();
            if confirm_password != password {
                eprintln!("Unmatched Master Key. Try Again...\n");
            } else {
                break;
            }
        }
    }
    password = adjust_password_length(&password, 24);
    debug("Encrypting password to store in data");
    setup_sql(handle);
    handle.password = Some(password.clone());

    let encrypted = encrypt(
        format!("test{}", rand_string(32)),
        &mut GivMe {
            key: Some(password.clone()),
            sql_con: None,
            password: Some(password.clone()),
            os: None,
            username: None,
        },
    )
    .unwrap();

    let encrypted_final_key = base64::encode(encrypted);
    debug("Adding to database");
    save_to_sql(
        Credentials::new(
            String::from("secret_key"),
            encrypted_final_key,
            String::new(),
        ),
        handle,
    )
    .unwrap();
    //println!("{}", password);
    Ok(true)
}

/// As a security measure and shortcommings of encryption algorithms.
/// We need to adjust password to some specified length. This is done
/// by repeating password couple of times.
pub fn adjust_password_length(input_password: &str, length: usize) -> String {
    let mut proper_length_password = String::new();
    if input_password.len() > length {
        eprintln!("Slicing password to first length digits...");
        proper_length_password = input_password[..length].to_string();
    } else {
        loop {
            for single_char in input_password.chars() {
                proper_length_password.push(single_char);
                if proper_length_password.len() == length {
                    break;
                }
            }
            if proper_length_password.len() == length {
                break;
            }
        }
    }
    proper_length_password
}

/// Authenticate User by verifying user entered password
/// with decrypted password.
///
/// This function also does some random string decryption as dual
/// security measure.
pub fn ask_pass_and_extract_key(handle: &mut GivMe) -> Result<bool, bool> {
    eprint!("Enter your Master Key: ");
    std::io::stdout().flush().unwrap();
    let user_entered_pass = read_password().unwrap().trim().to_string();
    let proper_length_password = adjust_password_length(&user_entered_pass, 24);
    let encrypted_key = base64::decode(&get_from_sql("secret_key", handle)[0].value).unwrap();
    let decrypted_pass = decrypt(
        &encrypted_key,
        &GivMe {
            key: Some(proper_length_password.clone()),
            sql_con: None,
            password: Some(proper_length_password.clone()),
            os: None,
            username: None,
        },
    );

    if decrypted_pass.is_err() {
        eprintln!("Error: Invalid Password");
        std::process::exit(1);
    }
    let decrypted_pass = decrypted_pass.unwrap();

    debug(format!("Decrypted pass: {}", decrypted_pass).as_str());
    if decrypted_pass.contains("test") {
        handle.key = Some((&decrypted_pass)[4..32].to_string());
        handle.password = Some(proper_length_password);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// A sort of wrapper to `get_from_sql()`. This function take
/// care of all encryption and decryption needed to retreive
/// data from Sqlite.
pub fn give_credentials(key: String, handle: &mut GivMe) -> Option<Credentials> {
    let original_key = base64::encode(encrypt(key.clone(), handle).unwrap());
    let mut creds: Vec<Credentials> = get_from_sql(&original_key, handle);
    if creds.is_empty() {
        None
    } else {
        creds[0].key = key;
        creds[0].value = decrypt(&base64::decode(creds[0].value.clone()).unwrap(), handle)
            .unwrap()
            .trim_matches(char::from(0))
            .to_string();
        if let Some(info) = &creds[0].info {
            creds[0].info = Some(
                decrypt(&base64::decode(info).unwrap(), handle)
                    .unwrap()
                    .trim_matches(char::from(0))
                    .to_string(),
            );
            Some(creds[0].clone())
        } else {
            Some(creds[0].clone())
        }
    }
}

/// Checks if value already exist in Sqlite
fn already_exist_in_sql(key: String, handle: &mut GivMe) -> Result<bool, sqlite::Error> {
    match handle
        .sql_con
        .as_ref()
        .unwrap()
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

/// Prints Credential struct to a user. Mainly used to
/// print Credential from user's query. Not for debugging.
pub fn show_credentials(cred: &Credentials) {
    println!("\nHere's your '{}':  {}", cred.key, cred.value);
    if cred.info != None {
        println!("Note: {}", cred.info.as_ref().unwrap());
    }
}

/// Save credentails to Sqlite. Somewhat like a wrapper to
/// `save_to_sql()` but this take care of all encryption and
/// decryption to save anything to database.
pub fn save_credentials(mut cred: Credentials, handle: &mut GivMe) -> Result<bool, String> {
    cred.key = base64::encode(encrypt(cred.key, handle).unwrap());
    match already_exist_in_sql(cred.key.clone(), handle) {
        Ok(exist) => {
            if exist {
                return Err("Key Already Exist. Choose some other name.".to_string());
            } else {
                cred.value = base64::encode(encrypt(cred.value, handle).unwrap());
                if let Some(info) = cred.info {
                    cred.info = Some(base64::encode(encrypt(info, handle).unwrap()));
                }

                Ok(save_to_sql(cred, handle).is_ok())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Encrypt given data with randomly generated string and
/// user's master key. 2 Encryption algorithms are used
/// TwoFish and TrippleDES.
pub fn encrypt(data: String, handle: &mut GivMe) -> Result<Vec<u8>, String> {
    let mut data_length: usize = 0;
    let mut filler = String::new();
    if handle.key.is_none() {
        ask_pass_and_extract_key(handle).unwrap();
    }

    for i in 1.. {
        if (i * Twofish::BLOCK_SIZE) >= data.chars().count() {
            data_length = i * Twofish::BLOCK_SIZE;
            break;
        }
    }
    debug(
        format!(
            "Data suppied for encryption will fit in {} bytes",
            data_length
        )
        .as_str(),
    );

    for _ in 0..(data_length - data.chars().count()) {
        filler = [&filler, "\x00"].join("");
    }

    let final_data = format!("{}{}", &data, &filler);
    debug(format!("Data to encrypt with filler: {:?}", final_data.as_bytes()).as_str());
    let mut one_step_encrypted: Vec<u8> = Vec::with_capacity(data_length);
    let mut two_step_encrypted: Vec<u8> = Vec::with_capacity(data_length);

    for _ in 0..data_length {
        one_step_encrypted.push(1);
        two_step_encrypted.push(1);
    }

    /*
    println!(
        "{} {} {}",
        Twofish::KEY_SIZE,
        Twofish::BLOCK_SIZE,
        Des3::KEY_SIZE
    );
    println!(
        "{:?} {:?} {:?}",
        &one_step_encrypted[..],
        &final_data.as_bytes(),
        handle.key
    );
    */

    Twofish::with_encrypt_key(handle.key.clone().unwrap().as_bytes())
        .unwrap()
        .encrypt(&mut one_step_encrypted[..], &final_data.as_bytes());
    Des3::with_encrypt_key(&handle.password.clone().unwrap().as_bytes())
        .unwrap()
        .encrypt(&mut two_step_encrypted[..], &one_step_encrypted);
    debug(format!("{:?}", &two_step_encrypted[..]).as_str());
    Ok(two_step_encrypted)
}

/// Decrypt given data with randomly generated string and
/// user's master key. 2 algorithms are used
/// TwoFish and TrippleDES.
pub fn decrypt(data: &[u8], handle: &GivMe) -> Result<String, String> {
    let data_length: usize = data.len();
    //println!("{:?}", data);

    if data_length % Twofish::BLOCK_SIZE != 0 {
        println!("{} {}", data_length, Twofish::BLOCK_SIZE);
        return Err(String::from("Invalid data"));
    }

    debug(format!("Data suppied for dencryption is in {} bytes", data_length).as_str());
    let mut one_step_decrypted: Vec<u8> = Vec::with_capacity(data_length);
    let mut two_step_decrypted: Vec<u8> = Vec::with_capacity(data_length);

    for _ in 0..data_length {
        one_step_decrypted.push(1);
        two_step_decrypted.push(1);
    }

    //println!("{}{}", Twofish::KEY_SIZE, Twofish::BLOCK_SIZE);

    Des3::with_decrypt_key(handle.password.clone().unwrap().as_bytes())
        .unwrap()
        .decrypt(&mut one_step_decrypted[..], data);
    Twofish::with_decrypt_key(handle.key.clone().unwrap().as_bytes())
        .unwrap()
        .decrypt(&mut two_step_decrypted[..], &one_step_decrypted);
    debug(format!("{:?}", &two_step_decrypted[..]).as_str());
    match std::str::from_utf8(&two_step_decrypted) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub enum OperatingSystem {
    Windows,
    Linux,
    Other,
}

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
