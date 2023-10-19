pub const SECRET_TOKEN_KEY: &'static str = "SECRET_TOKEN_KEY";
pub const SECRET_CRYPT_KEY: &'static str = "SECRET_CRYPT_KEY";
pub const ASSET_USER_FOLDER: &'static str = "ASSET_USER_FOLDER";

pub const DB_NAME: &'static str = "Wallet_DB";
pub const MONGO_DB_URI: &'static str = "MONGODB_URI";

pub const USER_COL: &'static str = "user";

pub const EXPIRED_TOKEN_TIME: i64 = 1;
pub const AUTHORIZATION: &'static str = "Authorization";

// Error Message
pub const NOT_FOUND: &'static str = "Not found";
pub const EMAIL_EXITS: &'static str = "Email was exits";
pub const INVALID_TOKEN: &str = "Error validating JWT token";
pub const EXPIRED_TOKEN: &str = "Expired Token";
pub const BAD_REQUEST: &str = "Bad Request";
pub const SOME_THING_WENT_WRONG: &str = "Some thing went wrong";
pub const INVALID_EMAIL: &'static str = "Invalid email";
pub const EMAIL_EMPTY: &'static str = "Email cannot empty";
pub const PASSWORD_EMPTY: &'static str = "Password cannot empty";

// socket message

pub const SENDER_MESSAGE: &'static str = "SENDER_MESSAGE";
pub const JOIN_ROOM: &'static str = "JOIN_ROOM";
