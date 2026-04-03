use crate::error::{AppError, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ring::rand::{SecureRandom, SystemRandom};

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

    Ok((BASE64.encode(private_key), BASE64.encode(public_key)))
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
    let bytes = BASE64
        .decode(encrypted_message)
        .map_err(|e| AppError::Crypto(format!("Decryption failed: {}", e)))?;
    String::from_utf8(bytes).map_err(|e| AppError::Crypto(format!("Invalid UTF-8: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_generate_random_bytes() {
        let bytes1 = generate_random_bytes(32).expect("Should generate bytes");
        let bytes2 = generate_random_bytes(32).expect("Should generate bytes");

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2, "Random bytes should be different");
    }

    #[test]
    fn test_generate_random_bytes_different_lengths() {
        let bytes16 = generate_random_bytes(16).expect("Should generate 16 bytes");
        let bytes64 = generate_random_bytes(64).expect("Should generate 64 bytes");

        assert_eq!(bytes16.len(), 16);
        assert_eq!(bytes64.len(), 64);
    }

    #[test]
    fn test_generate_keypair() {
        let (private_key1, public_key1) = generate_keypair().expect("Should generate keypair");
        let (private_key2, public_key2) = generate_keypair().expect("Should generate keypair");

        // Keys should be base64 encoded
        assert!(BASE64.decode(&private_key1).is_ok());
        assert!(BASE64.decode(&public_key1).is_ok());

        // Different calls should produce different keys
        assert_ne!(private_key1, private_key2);
        assert_ne!(public_key1, public_key2);
    }

    #[test]
    fn test_hash_data() {
        let data1 = b"Hello, World!";
        let data2 = b"Hello, World!";
        let data3 = b"Different data";

        let hash1 = hash_data(data1);
        let hash2 = hash_data(data2);
        let hash3 = hash_data(data3);

        // Same data should produce same hash
        assert_eq!(hash1, hash2);

        // Different data should produce different hash
        assert_ne!(hash1, hash3);

        // Hash should be base64 encoded
        assert!(BASE64.decode(&hash1).is_ok());
    }

    #[test]
    fn test_hash_data_empty() {
        let hash = hash_data(b"");
        assert!(BASE64.decode(&hash).is_ok());
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_generate_message_id() {
        let id1 = generate_message_id();
        let id2 = generate_message_id();

        // IDs should be different
        assert_ne!(id1, id2);

        // IDs should be valid UUIDs
        assert!(uuid::Uuid::parse_str(&id1).is_ok());
        assert!(uuid::Uuid::parse_str(&id2).is_ok());
    }

    #[test_case("Hello, World!"; "simple message")]
    #[test_case(""; "empty message")]
    #[test_case("Special chars: 🚀 αβγ 中文"; "unicode message")]
    fn test_encrypt_decrypt_message(message: &str) {
        let public_key = "test_public_key";
        let private_key = "test_private_key";

        // Encrypt message
        let encrypted = encrypt_message(message, public_key).expect("Encryption should succeed");

        // Decrypt message
        let decrypted =
            decrypt_message(&encrypted, private_key).expect("Decryption should succeed");

        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_decrypt_invalid_base64() {
        let invalid_encrypted = "not_valid_base64!";
        let private_key = "test_private_key";

        let result = decrypt_message(invalid_encrypted, private_key);
        assert!(result.is_err());

        if let Err(AppError::Crypto(msg)) = result {
            assert!(msg.contains("Decryption failed"));
        } else {
            panic!("Expected Crypto error");
        }
    }

    #[test]
    fn test_decrypt_invalid_utf8() {
        // Create invalid UTF-8 bytes
        let invalid_utf8_bytes = vec![0xFF, 0xFE, 0xFD];
        let invalid_encrypted = BASE64.encode(&invalid_utf8_bytes);
        let private_key = "test_private_key";

        let result = decrypt_message(&invalid_encrypted, private_key);
        assert!(result.is_err());

        if let Err(AppError::Crypto(msg)) = result {
            assert!(msg.contains("Invalid UTF-8"));
        } else {
            panic!("Expected Crypto error");
        }
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"Consistency test data";
        let hash1 = hash_data(data);
        let hash2 = hash_data(data);

        // Multiple calls with same data should be consistent
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_base64_encoding_decoding() {
        let original_data = b"Test data for base64";
        let encoded = BASE64.encode(original_data);
        let decoded = BASE64.decode(&encoded).expect("Should decode successfully");

        assert_eq!(original_data, decoded.as_slice());
    }
}
