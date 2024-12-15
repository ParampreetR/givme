use crate::models::credentials::Credentials;
use crate::models::givme::GivMe;
use crate::save_to_sql;
use crate::{adjust_password_length, get_from_sql, io::*};
use nettle::cipher::{Cipher, Des3, Twofish};
use rpassword::read_password;
use std::io::Write;
use std::process::exit;
use std::{fs, io};

/// Encrypt a file with 2 step encryption. File can be decrypted
/// on same computer on which it was encrypted.
///
/// ~This function do not empose any restrictions on size of file.
/// On large files, use at own risk
pub fn encrypt_file(in_path: String, out_path: String, handle: &mut GivMe) -> io::Result<()> {
    fs::write(&out_path, "".to_string())?; /* Check if we have privileges to write to target dest */
    let data = std::fs::read(&in_path)?;
    let encrypted_data = base64::encode(encrypt(base64::encode(data), handle).unwrap());
    fs::write(&out_path, encrypted_data)
}

/// Encrypt a file with 2 step decryption. File should be encrypted
/// on same computer on which it needs to decrypted.
///
/// ~This function do not empose any restrictions on size of file.
/// On large files, use at own risk
pub fn decrypt_file(in_path: String, out_path: String, handle: &mut GivMe) -> io::Result<()> {
    fs::write(&out_path, "".to_string())?; /* Check if we have privileges to write to target dest */
    let data = std::fs::read(&in_path)?;
    let decrypted_data =
        base64::decode(decrypt(&base64::decode(data).unwrap(), handle).unwrap()).unwrap();
    fs::write(&out_path, decrypted_data)
}

/// Retrieve secret key from the database
/// This key is used in encryption process with one encryption standard and
/// another key will be given by user
pub fn get_secret_key(handle: &mut GivMe) -> String {
    let encrypted_key = base64::decode(&get_from_sql("secret_key", handle)[0].value).unwrap();
    let decrypted_pass = decrypt(&encrypted_key, handle);

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
pub fn set_secret_key(handle: &mut GivMe) {
    let mut key;
    let mut confirm_key;
    let mut option: String = "".to_string();
    loop {
        print!("Enter Secret Key: ");
        key = handle.password.clone().unwrap();
        debug("Adjusting key size");

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
                confirm_key = adjust_password_length(&key, 32);
            } else {
                println!("Exiting...");
                exit(0)
            }
        } else {
            println!("Exiting...");
            exit(0)
        }
    }

    let encrypted = encrypt(confirm_key, handle).unwrap();

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
}

/// Encrypt given data with randomly generated string and
/// user's master key. 2 Encryption algorithms are used
/// TwoFish and 3DES.
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
        Ok(v) => Ok(v.trim_matches(char::from(0)).to_string()),
        Err(e) => Err(e.to_string()),
    }
}
