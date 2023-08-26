use crate::routes::accounts::login::LoginResponseAccountDetails;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;

// algorithm used for signing
#[derive(Debug, Serialize, Deserialize)]
pub enum TokenSigningAlgorithm {
    HS256,
}

// token type - eg. JWT
#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    JWT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHeader {
    pub alg: TokenSigningAlgorithm,
    pub typ: TokenType,
}

impl TokenHeader {
    pub fn new(alg: TokenSigningAlgorithm, typ: TokenType) -> Self {
        Self { alg, typ }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: String,
    pub role: String,
    pub uuid: String,
    pub nonce: String,
    pub expiry: i64, // timestamp of expiration
}

impl TokenClaims {
    pub fn new(user_id: String, role: String, uuid: String, nonce: &str, expiry: i64) -> Self {
        Self {
            user_id,
            role,
            uuid,
            nonce: nonce.to_string(), // TODO see if we need this nonce here...
            expiry,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    // pub claims: TokenClaims,
    // pub header: TokenHeader,
    pub user_id: String,
    pub role: String,
    pub uuid: String,
    pub nonce: String,
    pub exp: i64, // timestamp of expiration
}

impl Token {
    pub fn generate(&self) -> Option<String> {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(crate::private::JWT_SECRET.as_ref()),
        )
        .unwrap();

        if token.is_empty() {
            None
        } else {
            Some(token)
        }
    }
}

pub fn generate_session_id() -> Uuid {
    Uuid::new_v4()
}

pub fn generate_token(
    user_data: &LoginResponseAccountDetails,
    uuid: &Uuid,
    nonce: &str,
) -> Option<String> {
    let uuid_string = uuid.to_string();
    //let token_header = TokenHeader::new(TokenSigningAlgorithm::HS256, TokenType::JWT);
    // let token_claims = TokenClaims::new(
    //     user_data.id.to_string(),
    //     user_data.role.to_string(),
    //     uuid_string,
    //     nonce,
    //     1672563600,
    // );

    Token {
        // claims: token_claims,
        // header: token_header,
        user_id: user_data.id.to_string(),
        role: user_data.role.to_string(),
        uuid: uuid_string,
        nonce: nonce.to_string(),
        exp: 1672563600,
    }
    .generate()
}