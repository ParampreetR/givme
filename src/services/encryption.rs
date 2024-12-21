use crate::utils;
use log::debug;
use nettle::cipher::{Cipher, Des3, Twofish};
use rpassword::read_password;
use std::io::Write;
use std::{fs, io};

pub trait EncryptionService {
    fn new(password: String, key: String) -> impl EncryptionService {
        Encryption { key, password }
    }
    fn encrypt_file(&self, in_path: &str, out_path: &str) -> io::Result<()>;
    fn decrypt_file(&self, in_path: String, out_path: String) -> io::Result<()>;
    fn decrypt_secret_key(&self, secret_key: String) -> String;
    fn encrypt_secret_key(&self) -> Option<String>;
    fn encrypt(&self, data: String) -> Result<Vec<u8>, nettle::Error>;
    fn decrypt(&self, data: &[u8]) -> anyhow::Result<String, anyhow::Error>;
}

pub struct Encryption {
    password: String,
    key: String,
}

impl EncryptionService for Encryption {
    /// Encrypt a file with 2 step encryption. File can be decrypted
    /// on same computer on which it was encrypted.
    ///
    /// ~This function do not empose any restrictions on size of file.
    /// On large files, use at own risk
    fn encrypt_file(&self, in_path: &str, out_path: &str) -> io::Result<()> {
        fs::write(&out_path, "".to_string())?; /* Check if we have privileges to write to target dest */
        let data = std::fs::read(in_path)?;
        let encrypted_data = base64::encode(self.encrypt(base64::encode(data)).unwrap());
        fs::write(&out_path, encrypted_data)
    }

    /// Encrypt a file with 2 step decryption. File should be encrypted
    /// on same computer on which it needs to decrypted.
    ///
    /// ~This function do not empose any restrictions on size of file.
    /// On large files, use at own risk
    fn decrypt_file(&self, in_path: String, out_path: String) -> io::Result<()> {
        fs::write(&out_path, "".to_string())?; /* Check if we have privileges to write to target dest */
        let data = std::fs::read(&in_path)?;
        let decrypted_data =
            base64::decode(self.decrypt(&base64::decode(data).unwrap()).unwrap()).unwrap();
        fs::write(&out_path, decrypted_data)
    }

    /// Retrieve secret key from the database
    /// This key is used in encryption process with one encryption standard and
    /// another key will be given by user
    fn decrypt_secret_key(&self, secret_key: String) -> String {
        let encrypted_key = base64::decode(secret_key).unwrap();
        let decrypted_pass = self.decrypt(&encrypted_key);

        if decrypted_pass.is_err() {
            eprintln!("Error: Invalid Password");
            std::process::exit(1);
        }
        let decrypted_pass = decrypted_pass.unwrap();
        return decrypted_pass;
    }

    /// Store / replace secret key to the database
    ///
    /// This key is used in encryption process with one encryption standard and
    /// another key will be given by user when command will run
    fn encrypt_secret_key(&self) -> Option<String> {
        let mut key;
        let mut confirm_key;
        let mut option: String = "".to_string();
        loop {
            print!("Enter Secret Key: ");
            key = self.password.clone();
            debug!("Adjusting key size");

            print!("Confirm your Secret Key: ");
            std::io::stdout().flush().unwrap();
            confirm_key = read_password().unwrap().trim().to_string();
            if confirm_key != key {
                eprintln!("Unmatched Secret Key. Try Again...\n");
            } else {
                break;
            }
        }

        if key.len() < 32 {
            println!("Secret Key provided is shorter than 32 characters");
            print!("Auto Resize? (y/n) ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut option).unwrap();
            if option.chars().next().is_some() {
                if option.to_lowercase().chars().next().unwrap() == 'y' {
                    confirm_key = utils::helpers::adjust_password_length(&key, 32);
                } else {
                    println!("Exiting...");
                    return None;
                }
            } else {
                println!("Exiting...");
                return None;
            }
        }

        let encrypted = self.encrypt(confirm_key).unwrap();

        let encrypted_final_key = base64::encode(encrypted);
        debug!("Adding to database");
        Some(encrypted_final_key)
    }

    /// Encrypt given data with randomly generated string and
    /// user's master key. 2 Encryption algorithms are used
    /// TwoFish and 3DES.
    fn encrypt(&self, data: String) -> Result<Vec<u8>, nettle::Error> {
        let mut data_length: usize = 0;
        let mut filler = String::new();

        for i in 1.. {
            if (i * Twofish::BLOCK_SIZE) >= data.chars().count() {
                data_length = i * Twofish::BLOCK_SIZE;
                break;
            }
        }
        debug!(
            "Data suppied for encryption will fit in {} bytes",
            data_length
        );

        for _ in 0..(data_length - data.chars().count()) {
            filler = [&filler, "\x00"].join("");
        }

        let final_data = format!("{}{}", &data, &filler);
        debug!("Data to encrypt with filler: {:?}", final_data.as_bytes());
        let mut one_step_encrypted: Vec<u8> = Vec::with_capacity(data_length);
        let mut two_step_encrypted: Vec<u8> = Vec::with_capacity(data_length);

        for _ in 0..data_length {
            one_step_encrypted.push(1);
            two_step_encrypted.push(1);
        }

        Twofish::with_encrypt_key(self.key.clone().as_bytes())?
            .encrypt(&mut one_step_encrypted[..], &final_data.as_bytes());
        Des3::with_encrypt_key(&self.password.clone().as_bytes())?
            .encrypt(&mut two_step_encrypted[..], &one_step_encrypted);
        debug!("{:?}", &two_step_encrypted[..]);
        Ok(two_step_encrypted)
    }

    /// Decrypt given data with randomly generated string and
    /// user's master key. 2 algorithms are used
    /// TwoFish and TrippleDES.
    fn decrypt(&self, data: &[u8]) -> anyhow::Result<String, anyhow::Error> {
        let data_length: usize = data.len();
        //println!("{:?}", data);

        if data_length % Twofish::BLOCK_SIZE != 0 {
            println!("{} {}", data_length, Twofish::BLOCK_SIZE);
            return Err(anyhow::anyhow!(
                "Invalid data length: not a mulitple of Twofish block size {}",
                Twofish::BLOCK_SIZE
            ));
        }

        debug!("Data suppied for dencryption is in {} bytes", data_length);
        let mut one_step_decrypted: Vec<u8> = Vec::with_capacity(data_length);
        let mut two_step_decrypted: Vec<u8> = Vec::with_capacity(data_length);

        for _ in 0..data_length {
            one_step_decrypted.push(1);
            two_step_decrypted.push(1);
        }

        Des3::with_decrypt_key(self.password.as_bytes())?
            .decrypt(&mut one_step_decrypted[..], data);
        Twofish::with_decrypt_key(self.key.as_bytes())?
            .decrypt(&mut two_step_decrypted[..], &one_step_decrypted);
        debug!("{:?}", &two_step_decrypted[..]);
        match std::str::from_utf8(&two_step_decrypted) {
            Ok(v) => Ok(v.trim_matches(char::from(0)).to_string()),
            Err(e) => Err(e.into()),
        }
    }
}
