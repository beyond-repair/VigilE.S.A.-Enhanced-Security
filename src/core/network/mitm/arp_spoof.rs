//! Historical filename preserved: `arp_spoof.rs`.
//!
//! # NOT IMPLEMENTED / NOT FOR USE
//!
//! This module intentionally provides **no** ARP spoofing, traffic interception,
//! or MITM capability. All methods refuse. Do not expand this into an attack tool.

use std::net::IpAddr;

/// Refusal report returned instead of any interception result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitmReport {
    pub intercepted_packets: usize,
    pub policy_violations: bool,
    pub message: String,
}

/// Stub simulator that always refuses.
pub struct MitmSimulator;

impl MitmSimulator {
    pub fn new() -> Self {
        Self
    }

    /// Always returns an empty refusal report. Never intercepts traffic.
    pub async fn simulate(&self, _target: IpAddr) -> MitmReport {
        MitmReport {
            intercepted_packets: 0,
            policy_violations: false,
            message: "NOT IMPLEMENTED / NOT FOR USE — ARP spoofing and MITM are refused"
                .into(),
        }
    }

    /// Always empty. Never performs ARP spoofing.
    pub async fn intercept_traffic(&self, _target: IpAddr) -> Vec<()> {
        Vec::new()
    }
}

impl Default for MitmSimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn arp_spoof_stub_refuses() {
        let sim = MitmSimulator::new();
        let report = sim.simulate(IpAddr::V4(Ipv4Addr::LOCALHOST)).await;
        assert_eq!(report.intercepted_packets, 0);
        assert!(report.message.contains("NOT FOR USE"));
        assert!(sim.intercept_traffic(IpAddr::V4(Ipv4Addr::LOCALHOST)).await.is_empty());
    }
}
