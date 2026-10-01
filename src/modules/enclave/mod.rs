//! Confidential computing sketch — **local mock** (Claim-0).
//!
//! No Intel SGX / AMD SEV. "Seal" prepends a marker and stores bytes in memory.

use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SgxError {
    #[error("mock enclave error: {0}")]
    Mock(String),
}

/// Local stand-in for an SGX enclave.
pub struct SgxEnclave {
    eid: u64,
    sealed_data: Mutex<Vec<u8>>,
}

impl SgxEnclave {
    pub fn new() -> Result<Self, SgxError> {
        Ok(Self {
            eid: 1,
            sealed_data: Mutex::new(Vec::new()),
        })
    }

    pub fn process(&self, data: &[u8]) -> Result<Vec<u8>, SgxError> {
        let mut sealed = self
            .sealed_data
            .lock()
            .map_err(|_| SgxError::Mock("lock poisoned".into()))?;
        sealed.clear();
        sealed.extend_from_slice(b"MOCK-SEAL:");
        sealed.extend_from_slice(data);
        sealed.extend_from_slice(&self.eid.to_le_bytes());
        Ok(sealed.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_seal_roundtrip_prefix() {
        let enc = SgxEnclave::new().unwrap();
        let out = enc.process(b"secret").unwrap();
        assert!(out.starts_with(b"MOCK-SEAL:"));
        assert!(out.windows(6).any(|w| w == b"secret"));
    }
}
