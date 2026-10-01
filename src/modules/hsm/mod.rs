//! Hardware Security Module sketch — **local mock only** (Claim-0).
//!
//! Historical code referenced AWS KMS. This implementation never calls AWS;
//! it uses an in-process SHA-256 fingerprint for demo "signing".

use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HsmError {
    #[error("local mock HSM error: {0}")]
    Local(String),
}

/// In-memory crypto engine standing in for HSM/KMS.
pub struct HsmCryptoEngine {
    provider: String,
    key_arn: String,
}

impl HsmCryptoEngine {
    /// Construct a local mock engine (sync — no cloud config load).
    pub fn new_local(provider: &str, key_arn: &str) -> Self {
        Self {
            provider: provider.to_string(),
            key_arn: key_arn.to_string(),
        }
    }

    /// Historical async constructor name; resolves immediately to a local mock.
    pub async fn new(key_arn: &str) -> Self {
        Self::new_local("local_mock", key_arn)
    }

    pub fn status(&self) -> String {
        format!(
            "local_mock(provider={}, key={})",
            self.provider,
            short_arn(&self.key_arn)
        )
    }

    /// Demo "sign": HMAC-like SHA-256 over key_arn || data (not a real HSM).
    pub async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, HsmError> {
        Ok(self.fingerprint_bytes(data))
    }

    pub fn fingerprint(&self, data: &[u8]) -> String {
        hex::encode(self.fingerprint_bytes(data))
    }

    fn fingerprint_bytes(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.key_arn.as_bytes());
        hasher.update(b"|");
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

fn short_arn(arn: &str) -> &str {
    arn.rsplit('/').next().unwrap_or(arn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sign_is_deterministic() {
        let hsm = HsmCryptoEngine::new("demo-key").await;
        let a = hsm.sign(b"hello").await.unwrap();
        let b = hsm.sign(b"hello").await.unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 32);
    }

    #[test]
    fn status_mentions_local_mock() {
        let hsm = HsmCryptoEngine::new_local("local_mock", "arn:aws:kms:...:key/abcd");
        assert!(hsm.status().contains("local_mock"));
        assert!(hsm.status().contains("abcd"));
    }
}
