//! Confidential Computing with SGX Enclaves
//!
//! This module provides secure execution environments for sensitive operations.

use sgx_types::*;
use std::sync::Mutex;

/// SGX Enclave
pub struct SgxEnclave {
    eid: sgx_enclave_id_t,
    sealed_data: Mutex<Vec<u8>>,
}

impl SgxEnclave {
    /// Initialize a new SGX enclave
    pub fn new() -> Result<Self, SgxError> {
        let eid = unsafe { sgx_create_enclave()? };
        Ok(Self {
            eid,
            sealed_data: Mutex::new(Vec::new()),
        })
    }

    /// Securely process data inside the enclave
    pub fn process(&self, data: &[u8]) -> Result<Vec<u8>, SgxError> {
        let mut sealed = self.sealed_data.lock().unwrap();
        unsafe { sgx_seal_data(self.eid, data, &mut sealed)? };
        Ok(sealed.clone())
    }
}
