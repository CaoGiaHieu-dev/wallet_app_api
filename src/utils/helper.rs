use chrono::Utc;
use email_address::*;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use magic_crypt::MagicCryptTrait;
use mongodb::bson::oid::ObjectId;

use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{models::token_model::Claims, utils::constants};

pub fn get_current_time() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch: u128 = start.duration_since(UNIX_EPOCH).ok().unwrap().as_millis();
    return since_the_epoch;
}

pub fn validate_email(data: String) -> bool {
    return EmailAddress::is_valid(&data);
}

pub fn encryption(data: String) -> String {
    let m_crypt = new_magic_crypt!(env::var(constants::SECRET_CRYPT_KEY).unwrap(), 256);
    let encrypted_string = m_crypt.encrypt_str_to_base64(data);
    return String::from(encrypted_string);
}

pub fn decryption(data: String) -> String {
    let m_crypt = new_magic_crypt!(env::var(constants::SECRET_CRYPT_KEY).unwrap(), 256);
    let decrypted_string = m_crypt.decrypt_base64_to_string(data).unwrap();
    return String::from(decrypted_string);
}

pub fn create_jwt(id: ObjectId) -> Result<String, Error> {
    let secret = env::var(constants::SECRET_TOKEN_KEY).expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(constants::EXPIRED_TOKEN_TIME))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var(constants::SECRET_TOKEN_KEY).expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}
