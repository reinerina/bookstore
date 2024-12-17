use crate::{AES_TOKEN_KEY, COMPANY_NAME};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use rand::Rng;
use ring::aead;
use std::cmp::Ordering;
use std::ops::RangeFrom;

const TOKEN_VALIDITY: i64 = 60 * 60 * 24;

pub struct Token {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

pub async fn generate_token(username: &str) -> anyhow::Result<Token> {
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, AES_TOKEN_KEY).unwrap();
    let sealing_key = aead::LessSafeKey::new(key);

    let date = Utc::now();

    let date = date + chrono::Duration::seconds(TOKEN_VALIDITY);

    let date = date.to_rfc3339();

    let date = date.as_bytes();

    let date_buffer = &mut [0u8; 32];
    date_buffer[..date.len()].copy_from_slice(date);

    let username = username.as_bytes();

    let company_name = COMPANY_NAME.as_slice();

    let mut raw_token = [date_buffer, company_name, username].concat();

    let mut rg = rand::thread_rng();
    let mut nonce_buffer = [0u8; 12];
    rg.fill(&mut nonce_buffer);
    let nonce = aead::Nonce::assume_unique_for_key(nonce_buffer);

    match sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::empty(), &mut raw_token) {
        Ok(tag) => Ok(Token {
            token: BASE64_STANDARD.encode(&raw_token),
            tag: BASE64_STANDARD.encode(tag.as_ref()),
            nonce: BASE64_STANDARD.encode(nonce_buffer),
        }),
        Err(_) => anyhow::bail!("failed to generate token"),
    }
}

pub async fn decrypt_token(token: &Token) -> anyhow::Result<(String, DateTime<Utc>)> {
    let tag = &token.tag;
    let nonce = &token.nonce;
    let token = &token.token;
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, AES_TOKEN_KEY).unwrap();
    let opening_key = aead::LessSafeKey::new(key);

    let mut token = BASE64_STANDARD.decode(token)?;
    let tag = BASE64_STANDARD.decode(tag)?;
    let mut tag_buffer = [0u8; 16];
    tag_buffer.copy_from_slice(tag.as_slice());
    let tag = aead::Tag::from(tag_buffer);
    let nonce = BASE64_STANDARD.decode(nonce)?;
    let mut nonce_buffer = [0u8; 12];
    nonce_buffer.copy_from_slice(nonce.as_slice());
    let nonce = aead::Nonce::assume_unique_for_key(nonce_buffer);

    match opening_key.open_in_place_separate_tag(
        nonce,
        aead::Aad::empty(),
        tag,
        &mut token,
        RangeFrom { start: 0 },
    ) {
        Ok(token) => {
            let token = String::from_utf8_lossy(&token).to_string();
            let date = DateTime::parse_from_rfc3339(&token[..32])?;

            Ok((token[41..].to_string(), DateTime::from(date)))
        }
        Err(e) => anyhow::bail!("invalid token : {}", e),
    }
}

pub async fn validate_token(token: &Token) -> anyhow::Result<String> {
    match decrypt_token(token).await {
        Ok((username, date)) => {
            let now = Utc::now();
            match now.timestamp().cmp(&date.timestamp()) {
                Ordering::Less => Ok(username),
                _ => anyhow::bail!("token expired"),
            }
        }
        Err(e) => anyhow::bail!(e),
    }
}
