//! Historical filename preserved: `cracker.rs`.
//!
//! # NOT IMPLEMENTED / NOT FOR USE
//!
//! This module intentionally provides **no** password cracking, dictionary
//! attacks, or credential recovery. Methods return empty results and document
//! refusal. Do not expand this into an attack tool.

use crate::modules::hsm::HsmCryptoEngine;
use std::sync::Arc;

/// Empty result type — never populated with cracked credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedPassword {
    pub hash: String,
    pub plaintext: Option<String>,
}

/// Stub auditor that always refuses.
pub struct PasswordAuditor {
    _hsm: Arc<HsmCryptoEngine>,
    _wordlists: Vec<String>,
}

impl PasswordAuditor {
    pub fn new(hsm: Arc<HsmCryptoEngine>) -> Self {
        Self {
            _hsm: hsm,
            _wordlists: Vec::new(),
        }
    }

    /// Always returns an empty list. Never attempts to crack passwords.
    pub async fn crack(&self, _hashes: Vec<String>) -> Vec<CrackedPassword> {
        Vec::new()
    }

    /// Always empty. Dictionary attack is not implemented and will not be.
    pub async fn dictionary_attack(&self, _hash: &str) -> Vec<CrackedPassword> {
        Vec::new()
    }

    pub fn refusal_message() -> &'static str {
        "NOT IMPLEMENTED / NOT FOR USE — password cracking is refused"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cracker_stub_returns_empty() {
        let hsm = Arc::new(HsmCryptoEngine::new_local("local_mock", "k"));
        let auditor = PasswordAuditor::new(hsm);
        let out = auditor
            .crack(vec!["deadbeef".into(), "cafebabe".into()])
            .await;
        assert!(out.is_empty());
        assert!(PasswordAuditor::refusal_message().contains("NOT FOR USE"));
        assert!(auditor.dictionary_attack("x").await.is_empty());
    }
}
