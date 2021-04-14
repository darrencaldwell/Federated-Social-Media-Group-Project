use jsonwebtoken::{decode, encode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct Claim {
    username: String,
    user_id: String,
    exp: i64,
}

/// The format of the JWT when decrypted
impl Claim {
    fn new(user_id: String, username: String) -> Self {
        Self{
            user_id,
            username,
            exp: (Utc::now() + Duration::days(2)).timestamp(),
        }
    }
}

/// Given a user_id and username, create a new JWT
pub fn encode_jwt(user_id: String, username: String) -> Result<String> {
    let claim = Claim::new(user_id, username);
    let key = EncodingKey::from_secret("change_me".as_bytes());
    Ok(encode(&Header::default(), &claim, &key)?)
}

/// Decode a JWT, returning the user_id if valid
pub fn decode_jwt(token: &str) -> Result<String> {
    let key = DecodingKey::from_secret("change_me".as_bytes());
    let user_id = decode::<Claim>(token, &key, &Validation::default())
                    .map(|data| data.claims.user_id)?;
    Ok(user_id)
}

/// Decode a JWT, returning the expiration date if valid
pub fn decode_jwt_expr(token: &str) -> Result<i64> {
    let key = DecodingKey::from_secret("change_me".as_bytes());
    let expr_date = decode::<Claim>(token, &key, &Validation::default())
                    .map(|data| data.claims.exp)?;
    Ok(expr_date)
}
