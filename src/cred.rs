use crate::encryption::{decrypt, encrypt};
use crate::sql::*;
use crate::structs::{Credentials, GivMe};

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
      .to_string();
    if let Some(info) = &creds[0].info {
      creds[0].info = Some(
        decrypt(&base64::decode(info).unwrap(), handle)
          .unwrap()
          .to_string(),
      );
      Some(creds[0].clone())
    } else {
      Some(creds[0].clone())
    }
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

pub fn delete_credentails(mut key: String, handle: &mut GivMe) -> Result<bool, String> {
  key = base64::encode(encrypt(key, handle).unwrap());
  match del_from_sql(key, handle) {
    Ok(_r) => Ok(true),
    Err(err) => Err(err.to_string()),
  }
}
