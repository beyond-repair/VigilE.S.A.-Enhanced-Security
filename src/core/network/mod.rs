//! Network security module — local mock monitor (Claim-0).
//!
//! Historical sketch mentioned zero-copy packet processing and Zero Trust.
//! This Claim-0 path only simulates heartbeats; it does not sniff or filter traffic.

pub mod mitm;

use crate::modules::hsm::HsmCryptoEngine;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Mock network monitor driven by config (no packet capture).
pub struct NetworkMonitor {
    hsm: Arc<HsmCryptoEngine>,
    interface: String,
    packets_per_tick: u32,
    ticks: AtomicU64,
}

impl NetworkMonitor {
    pub fn new(hsm: Arc<HsmCryptoEngine>, interface: String, packets_per_tick: u32) -> Self {
        Self {
            hsm,
            interface,
            packets_per_tick,
            ticks: AtomicU64::new(0),
        }
    }

    /// Start is a no-op for the sketch (kept for API familiarity with historical main.rs).
    pub async fn start(&self) {
        let _ = self.heartbeat().await;
    }

    pub async fn heartbeat(&self) -> String {
        let n = self.ticks.fetch_add(1, Ordering::Relaxed) + 1;
        let fingerprint = self.hsm.fingerprint(self.interface.as_bytes());
        format!(
            "ok(iface={}, mock_pkts={}, tick={}, hsm_fp={})",
            self.interface, self.packets_per_tick, n, &fingerprint[..8.min(fingerprint.len())]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_heartbeat_ok() {
        let hsm = Arc::new(HsmCryptoEngine::new_local("local_mock", "demo-key"));
        let mon = NetworkMonitor::new(hsm, "lo".into(), 2);
        let status = mon.heartbeat().await;
        assert!(status.starts_with("ok("));
    }
}
