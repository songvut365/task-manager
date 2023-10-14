use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug, Serialize, Deserialize)]

struct ClaimsToken {
    user_id: String,
}

#[allow(unused)]
pub fn get_user_id(token: &str) -> Result<String, Box<dyn Error>> {
    let secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(err) => {
            error!("get secret from env error: {:}", err);
            let default_secret = "Th1$!sS3cr3t".to_string();
            default_secret
        }
    };

    let token_message = decode::<ClaimsToken>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_message.claims.user_id)
}
