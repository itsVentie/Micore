use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionError,
    #[error("Decryption failed")]
    DecryptionError,
    #[error("Key derivation failed")]
    KeyDerivationError,
}

pub fn derive_key(passphrase: &str, salt: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|_| CryptoError::KeyDerivationError)?;
    Ok(key)
}

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::EncryptionError)?;
    let nonce_bytes = aes_gcm::aead::OsRng;
    let nonce = Nonce::from_slice(&[0u8; 12]); 
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CryptoError::EncryptionError)?;
        
    Ok((ciphertext, nonce.to_vec()))
} 

pub fn decrypt(key: &[u8; 32], ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if ciphertext_with_nonce.len() < 12 {
        return Err(CryptoError::DecryptionError);
    }
    let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::DecryptionError)?;
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionError)
}