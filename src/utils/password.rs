use crate::{AES_ADMIN_PWD_KEY, AES_PWD_KEY};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ring::aead;

pub async fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, AES_PWD_KEY).unwrap();
    let sealing_key = aead::LessSafeKey::new(key);
    let nonce = aead::Nonce::assume_unique_for_key([0; 12]);

    let mut password = password.to_string();
    let mut password = unsafe { password.as_bytes_mut() };

    match sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::empty(), &mut password) {
        Ok(_) => Ok(BASE64_STANDARD.encode(&password)),
        Err(_) => Err(anyhow::anyhow!("failed to encrypt password")),
    }
}

pub async fn encrypt_admin_password(password: &str) -> anyhow::Result<String> {
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, AES_ADMIN_PWD_KEY).unwrap();
    let sealing_key = aead::LessSafeKey::new(key);
    let nonce = aead::Nonce::assume_unique_for_key([0; 12]);

    let mut password = password.to_string();
    let mut password = unsafe { password.as_bytes_mut() };

    match sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::empty(), &mut password) {
        Ok(_) => Ok(BASE64_STANDARD.encode(&password)),
        Err(_) => Err(anyhow::anyhow!("failed to encrypt password")),
    }
}
