use std::env;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::domain::entities::Jwt;

pub struct JwtEncoder {
    secret: String,
}

impl JwtEncoder {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set on ENV");

        Self { secret }
    }

    pub fn encode(&self, jwt: &Jwt) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &jwt,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn decode(&self, token: &str) -> Result<Jwt, jsonwebtoken::errors::Error> {
        let token_data = decode::<Jwt>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
