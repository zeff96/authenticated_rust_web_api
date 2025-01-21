use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;

use crate::model::Claim;

// generate access token
pub fn generate_access_token(user_id: &str, secret: &str) -> jsonwebtoken::errors::Result<String> {
    let claim = Claim {
        sub: user_id.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::seconds(5)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

//generate refresh token
pub fn generate_refresh_token(user_id: &str, secret: &str) -> jsonwebtoken::errors::Result<String> {
    let claim = Claim {
        sub: user_id.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::days(365)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

// decode tokens
pub fn decode_token(token: &str, secret: &str) -> jsonwebtoken::errors::Result<TokenData<Claim>> {
    let validation = Validation::default();
    decode::<Claim>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
}

//password hashing using argon2
pub fn generate_hash_password(password: &str) -> argon2::password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

//verify user password against database hashed password
pub fn verify_hashed_password(
    password: &str,
    db_password: &str,
) -> argon2::password_hash::Result<()> {
    let argon2 = Argon2::default();
    let stored_password = PasswordHash::new(db_password)?;
    argon2.verify_password(password.as_bytes(), &stored_password)
}

// parse uuid from string
pub fn parse_uuid(token: &str) -> Result<Uuid, actix_web::Error> {
    Uuid::try_parse(token)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error parsing uuid from string"))
}
