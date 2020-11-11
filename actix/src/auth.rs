use jsonwebtoken::{decode, encode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct Claim {
    username: String,
    exp: i64,
}

impl Claim {
    fn new(username: String) -> Self {
        Self{
            username,
            exp: (Utc::now() + Duration::days(1)).timestamp(),
        }
    }
}

pub fn encode_jwt(username: String) -> Result<String> {
    let claim = Claim::new(username);
    let key = EncodingKey::from_secret("change_me".as_ref());
    Ok(encode(&Header::default(), &claim, &key)?)
}

pub fn decode_jwt(token: &str) -> Result<String> {
    let key = DecodingKey::from_secret("change_me".as_ref());
    let username = decode::<Claim>(token, &key, &Validation::default())
                    .map(|data| data.claims.username)?;
    Ok(username)
}
