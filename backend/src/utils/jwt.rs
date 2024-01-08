use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct JWT;

impl JWT {
    // generating JWT Token
    pub fn generate_token<T>(payload: &T) -> Result<String>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        println!("{:#?}", payload);
        encode(
            &Header::default(),
            payload,
            &EncodingKey::from_secret("secret".as_bytes()),
        )
    }

    // validating JWT Token
    pub fn validate_token<T>(token: &str) -> Result<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let token_payload: TokenData<T> = decode(
            token,
            &DecodingKey::from_secret("secret".as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_payload.claims as T)
    }
}
