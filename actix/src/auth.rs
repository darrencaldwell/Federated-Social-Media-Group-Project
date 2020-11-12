use jsonwebtoken::{decode, encode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct Claim {
    username: String,
    user_id: u64,
    exp: i64,
}

impl Claim {
    fn new(user_id: u64, username: String) -> Self {
        Self{
            user_id,
            username,
            exp: (Utc::now() + Duration::days(1)).timestamp(),
        }
    }
}

pub fn encode_jwt(user_id: u64, username: String) -> Result<String> {
    let claim = Claim::new(user_id, username);
    let key = EncodingKey::from_secret("change_me".as_ref());
    Ok(encode(&Header::default(), &claim, &key)?)
}

pub fn decode_jwt(token: &str) -> Result<u64> {
    let key = DecodingKey::from_secret("change_me".as_ref());
    let user_id = decode::<Claim>(token, &key, &Validation::default())
                    .map(|data| data.claims.user_id)?;
    Ok(user_id)
}
