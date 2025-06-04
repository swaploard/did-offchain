use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, TokenData, Algorithm};
use serde::{Deserialize, Serialize};
use crate::settings::jwt::JWT_CONFIG;
use crate::models::claims::Claims;
use crate::models::user::UserRole;

pub fn issue_tokens(user_id: &str, role: UserRole) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let now = Utc::now();

    let access_exp = now
        .checked_add_signed(Duration::minutes(JWT_CONFIG.access_expiry_minutes))
        .unwrap()
        .timestamp() as usize;

    let access_claims = Claims {
        sub: user_id.to_string(),
        exp: access_exp,
        token_type: "access".into(),
        role: role.clone(),
    };

    let access_jwt = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(JWT_CONFIG.access_secret.as_ref()),
    )?;

    let refresh_exp = now
        .checked_add_signed(Duration::minutes(JWT_CONFIG.refresh_expiry_minutes))
        .unwrap()
        .timestamp() as usize;

    let refresh_claims = Claims {
        sub: user_id.to_string(),
        exp: refresh_exp,
        token_type: "refresh".into(),
        role,
    };

    let refresh_jwt = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(JWT_CONFIG.refresh_secret.as_ref()),
    )?;

    Ok((access_jwt, refresh_jwt))
}


pub fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256))
}