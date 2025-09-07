use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString};
use b64::{STANDARD, ToBase64};

pub fn crypto_password(password: &str) -> String {
    let salt = SaltString::from_b64(b"some salt".to_base64(STANDARD).as_str()).unwrap();

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    password_hash
}
