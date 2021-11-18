use crate::io::*;
use crate::structs::GivMe;
use nettle::cipher::{Cipher, Des3, Twofish};
use std::{fs, io};

/// Encrypt a file with 2 step encryption. File can be decrypted
/// on same computer on which it was encrypted.
///
/// ~This function do not empose any restrictions on size of file.
/// On large files, use at own risk
pub fn encrypt_file(in_path: String, out_path: String, handle: &mut GivMe) -> io::Result<()> {
  fs::write(&out_path, "".to_string())?; /* Check if we have privileges to write to target dest */
  let data = fs::read_to_string(&in_path)?;
  let encrypted_data = base64::encode(encrypt(data, handle).unwrap());
  fs::write(&out_path, encrypted_data)
}

/// Encrypt a file with 2 step decryption. File should be encrypted
/// on same computer on which it needs to decrypted.
///
/// ~This function do not empose any restrictions on size of file.
/// On large files, use at own risk
pub fn decrypt_file(in_path: String, out_path: String, handle: &mut GivMe) -> io::Result<()> {
  fs::write(&out_path, "".to_string())?; /* Check if we have privileges to write to target dest */
  let data = fs::read_to_string(&in_path)?;
  let decrypted_data = decrypt(&base64::decode(data).unwrap(), handle).unwrap();
  fs::write(&out_path, decrypted_data)
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
    Ok(v) => Ok(v.trim_matches(char::from(0)).to_string()),
    Err(e) => Err(e.to_string()),
  }
}
