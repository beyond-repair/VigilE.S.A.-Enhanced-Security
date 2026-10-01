//! eBPF-based network security sketch — **in-memory mock** (Claim-0).
//!
//! No `aya`, no kernel load, no bytecode. Policies live in a HashMap.

use std::collections::HashMap;
use std::net::Ipv4Addr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EbpfError {
    #[error("mock eBPF error: {0}")]
    Mock(String),
}

/// In-memory stand-in for an eBPF policy engine.
pub struct EbpfPolicyEngine {
    policy_map: HashMap<[u8; 4], [u8; 4]>,
}

impl EbpfPolicyEngine {
    /// Accepts bytes for API familiarity; does not load BPF bytecode.
    pub fn load(_program: &[u8]) -> Result<Self, EbpfError> {
        Ok(Self {
            policy_map: HashMap::new(),
        })
    }

    /// Record a mock src→dst policy entry in memory.
    pub fn apply_policy(&mut self, src: Ipv4Addr, dst: Ipv4Addr) -> Result<(), EbpfError> {
        self.policy_map.insert(src.octets(), dst.octets());
        Ok(())
    }

    pub fn policy_count(&self) -> usize {
        self.policy_map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_policy_without_bytecode_file() {
        // No deployments/ebpf/firewall.bpf required.
        let mut engine = EbpfPolicyEngine::load(b"MOCK-NOT-BPF").unwrap();
        engine
            .apply_policy(Ipv4Addr::new(10, 0, 0, 1), Ipv4Addr::new(10, 0, 0, 2))
            .unwrap();
        assert_eq!(engine.policy_count(), 1);
    }
}
