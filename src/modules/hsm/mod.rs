//! Hardware Security Module (HSM) Integration
//!
//! This module provides cryptographic operations using AWS KMS or Google Cloud KMS.

use aws_sdk_kms::Client;
use std::sync::Arc;

/// HSM Crypto Engine
pub struct HsmCryptoEngine {
    client: Client,
    key_arn: String,
}

impl HsmCryptoEngine {
    /// Create a new HSM Crypto Engine instance
    pub async fn new(key_arn: &str) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self {
            client,
            key_arn: key_arn.to_string(),
        }
    }

    /// Sign data using the HSM
    pub async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, HsmError> {
        let response = self.client
            .sign()
            .key_id(&self.key_arn)
            .message(data)
            .send()
            .await?;
        Ok(response.signature)
    }
}
