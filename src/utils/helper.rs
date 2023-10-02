use magic_crypt::MagicCryptTrait;

use std::time::{SystemTime, UNIX_EPOCH};

 const CRYPT_KEY: &str = "my private key";

pub fn get_current_time() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch: u128 = start.duration_since(UNIX_EPOCH).ok().unwrap().as_millis();
    return since_the_epoch;
}

pub fn encryption(data: String) -> String {
    let m_crypt = new_magic_crypt!(CRYPT_KEY, 256);
    let encrypted_string = m_crypt.encrypt_str_to_base64(data);
    return String::from(encrypted_string);
}

pub fn decryption(data: String) -> String {
    let m_crypt = new_magic_crypt!(CRYPT_KEY, 256);
    let decrypted_string = m_crypt.decrypt_base64_to_string(data).unwrap();
    return String::from(decrypted_string);
}
