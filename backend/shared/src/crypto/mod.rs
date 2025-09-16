use ring::rand::{SecureRandom, SystemRandom};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::error::{AppError, Result};

pub fn generate_random_bytes(length: usize) -> Result<Vec<u8>> {
    let rng = SystemRandom::new();
    let mut bytes = vec![0u8; length];
    rng.fill(&mut bytes)
        .map_err(|_| AppError::Crypto("Failed to generate random bytes".to_string()))?;
    Ok(bytes)
}

pub fn generate_keypair() -> Result<(String, String)> {
    // This is a simplified implementation
    // In a real application, you would use proper key generation libraries
    let private_key = generate_random_bytes(32)?;
    let public_key = generate_random_bytes(32)?;

    Ok((
        BASE64.encode(private_key),
        BASE64.encode(public_key),
    ))
}

pub fn hash_data(data: &[u8]) -> String {
    use ring::digest;
    let hash = digest::digest(&digest::SHA256, data);
    BASE64.encode(hash.as_ref())
}

pub fn generate_message_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

// Placeholder for E2EE functions
// In production, these would use proper encryption libraries like libsodium
pub fn encrypt_message(message: &str, _public_key: &str) -> Result<String> {
    // Placeholder implementation
    // In production: use proper E2EE encryption
    Ok(BASE64.encode(message.as_bytes()))
}

pub fn decrypt_message(encrypted_message: &str, _private_key: &str) -> Result<String> {
    // Placeholder implementation
    // In production: use proper E2EE decryption
    let bytes = BASE64.decode(encrypted_message)
        .map_err(|e| AppError::Crypto(format!("Decryption failed: {}", e)))?;
    String::from_utf8(bytes)
        .map_err(|e| AppError::Crypto(format!("Invalid UTF-8: {}", e)))
}