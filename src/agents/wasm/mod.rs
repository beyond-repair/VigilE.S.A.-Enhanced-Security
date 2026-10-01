//! Wasm security plugin sketch — **local mock** (Claim-0).
//!
//! No wasmtime. `run_policy` echoes a validation marker over the input.

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WasmError {
    #[error("mock wasm error: {0}")]
    Mock(String),
}

/// In-process stand-in for a Wasm policy runner.
pub struct WasmPolicyEngine {
    label: String,
}

impl WasmPolicyEngine {
    pub fn new() -> Self {
        Self {
            label: "mock-wasm".into(),
        }
    }

    /// Demo validate: returns `OK:` + input. Does not execute Wasm.
    pub fn run_policy(&self, _wasm_module: &[u8], input: &[u8]) -> Result<Vec<u8>, WasmError> {
        let mut out = format!("OK:{}:", self.label).into_bytes();
        out.extend_from_slice(input);
        Ok(out)
    }
}

impl Default for WasmPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_policy_echoes() {
        let eng = WasmPolicyEngine::new();
        let out = eng.run_policy(b"", b"ping").unwrap();
        assert!(out.starts_with(b"OK:mock-wasm:"));
        assert!(out.ends_with(b"ping"));
    }
}
