//! VigilE.S.A. Enhanced Security — Claim-0 library surface.
//!
//! Local in-memory mocks for NetworkMonitor, CloudShield, and HsmCryptoEngine.
//! Offensive-named modules (`arp_spoof`, `password_audit::cracker`) are safe
//! no-op stubs that refuse to perform attacks. No real eBPF, SGX, HSM, or cloud
//! APIs are invoked on the default path.

pub mod agents;
pub mod config;
pub mod core;
pub mod modules;

pub use config::SecurityConfig;
pub use core::cloud::CloudShield;
pub use core::network::NetworkMonitor;
pub use modules::hsm::HsmCryptoEngine;

use std::sync::Arc;
use std::time::Duration;

/// One security-loop tick: mock heartbeat from network + cloud + HSM status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    pub tick: u64,
    pub network_status: String,
    pub cloud_status: String,
    pub hsm_status: String,
}

/// Orchestrates the Claim-0 demo security loop (config-driven mocks only).
pub struct SecurityLoop {
    pub config: SecurityConfig,
    pub hsm: Arc<HsmCryptoEngine>,
    pub network: NetworkMonitor,
    pub cloud: CloudShield,
}

impl SecurityLoop {
    pub fn from_config(config: SecurityConfig) -> Self {
        let hsm = Arc::new(HsmCryptoEngine::new_local(
            &config.hsm.provider,
            &config.hsm.key_arn,
        ));
        let network = NetworkMonitor::new(
            Arc::clone(&hsm),
            config.network.interface.clone(),
            config.network.mock_packets_per_tick,
        );
        let cloud = CloudShield::new(
            Arc::clone(&hsm),
            config.cloud.aws_region.clone(),
            config.cloud.gcp_project.clone(),
            config.cloud.mock_resource_count,
        );
        Self {
            config,
            hsm,
            network,
            cloud,
        }
    }

    /// Run a single mock monitoring tick (suitable for `--once` / CI).
    pub async fn tick_once(&self, tick: u64) -> Heartbeat {
        let net = self.network.heartbeat().await;
        let cloud = self.cloud.heartbeat().await;
        let hsm = self.hsm.status();
        Heartbeat {
            tick,
            network_status: net,
            cloud_status: cloud,
            hsm_status: hsm,
        }
    }

    /// Run the demo loop for `max_heartbeats` ticks (or until cancelled).
    pub async fn run_demo(&self) -> Vec<Heartbeat> {
        let max = self.config.demo.max_heartbeats.max(1);
        let delay = Duration::from_secs(self.config.demo.heartbeat_secs.max(1));
        let mut out = Vec::with_capacity(max as usize);
        for i in 1..=max {
            out.push(self.tick_once(i).await);
            if i < max {
                tokio::time::sleep(delay).await;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn demo_loop_produces_heartbeats() {
        let cfg = SecurityConfig::demo_default();
        let loop_ = SecurityLoop::from_config(cfg);
        let beats = loop_.run_demo().await;
        assert_eq!(beats.len(), 3);
        assert_eq!(beats[0].tick, 1);
        assert!(beats[0].network_status.contains("ok"));
        assert!(beats[0].cloud_status.contains("ok"));
        assert!(beats[0].hsm_status.contains("local_mock"));
    }
}
