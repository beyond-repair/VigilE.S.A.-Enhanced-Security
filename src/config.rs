//! Load `config/security.toml` for the Claim-0 demo.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub demo: DemoSection,
    pub network: NetworkSection,
    pub cloud: CloudSection,
    pub hsm: HsmSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoSection {
    pub heartbeat_secs: u64,
    pub max_heartbeats: u64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSection {
    pub interface: String,
    pub xdp_program: String,
    #[serde(default = "default_packets")]
    pub mock_packets_per_tick: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudSection {
    pub aws_region: String,
    pub gcp_project: String,
    #[serde(default = "default_resources")]
    pub mock_resource_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmSection {
    pub provider: String,
    pub key_arn: String,
}

fn default_packets() -> u32 {
    2
}
fn default_resources() -> u32 {
    3
}

impl SecurityConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let text = fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    /// Built-in defaults used when no file is present (tests / fallback).
    pub fn demo_default() -> Self {
        Self {
            demo: DemoSection {
                heartbeat_secs: 1,
                max_heartbeats: 3,
                label: "vigil-esa-local-demo".into(),
            },
            network: NetworkSection {
                interface: "lo".into(),
                xdp_program: "deployments/ebpf/firewall.bpf".into(),
                mock_packets_per_tick: 2,
            },
            cloud: CloudSection {
                aws_region: "us-west-2".into(),
                gcp_project: "vigil-sec-demo".into(),
                mock_resource_count: 3,
            },
            hsm: HsmSection {
                provider: "local_mock".into(),
                key_arn: "arn:aws:kms:us-west-2:000000000000:key/demo-local".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_repo_security_toml() {
        let cfg = SecurityConfig::load("config/security.toml").expect("config");
        assert_eq!(cfg.hsm.provider, "local_mock");
        assert_eq!(cfg.demo.max_heartbeats, 3);
        assert_eq!(cfg.network.interface, "lo");
    }

    #[test]
    fn demo_default_is_self_consistent() {
        let cfg = SecurityConfig::demo_default();
        assert!(cfg.demo.max_heartbeats >= 1);
    }
}
