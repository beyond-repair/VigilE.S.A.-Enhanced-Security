//! Cloud security module — local mock shield (Claim-0).
//!
//! No AWS SDK or GCP client calls. Resource lists are synthetic.

use crate::modules::hsm::HsmCryptoEngine;
use std::sync::Arc;

/// Mock cloud protection engine.
pub struct CloudShield {
    hsm: Arc<HsmCryptoEngine>,
    aws_region: String,
    gcp_project: String,
    mock_resource_count: u32,
    protection_enabled: bool,
}

/// Synthetic cloud resource for demo scans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockResource {
    pub id: String,
    pub provider: String,
}

impl CloudShield {
    pub fn new(
        hsm: Arc<HsmCryptoEngine>,
        aws_region: String,
        gcp_project: String,
        mock_resource_count: u32,
    ) -> Self {
        Self {
            hsm,
            aws_region,
            gcp_project,
            mock_resource_count,
            protection_enabled: false,
        }
    }

    pub async fn enable_protection(&mut self) {
        self.protection_enabled = true;
    }

    pub fn list_resources(&self) -> Vec<MockResource> {
        (0..self.mock_resource_count)
            .map(|i| MockResource {
                id: format!("res-{}", i),
                provider: if i % 2 == 0 {
                    format!("aws:{}", self.aws_region)
                } else {
                    format!("gcp:{}", self.gcp_project)
                },
            })
            .collect()
    }

    pub async fn heartbeat(&self) -> String {
        let n = self.list_resources().len();
        let sig = self.hsm.fingerprint(self.gcp_project.as_bytes());
        format!(
            "ok(region={}, project={}, resources={}, enabled={}, hsm_fp={})",
            self.aws_region,
            self.gcp_project,
            n,
            self.protection_enabled,
            &sig[..8.min(sig.len())]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_cloud_lists_resources() {
        let hsm = Arc::new(HsmCryptoEngine::new_local("local_mock", "k"));
        let mut shield = CloudShield::new(hsm, "us-west-2".into(), "demo".into(), 3);
        shield.enable_protection().await;
        assert_eq!(shield.list_resources().len(), 3);
        let hb = shield.heartbeat().await;
        assert!(hb.contains("enabled=true"));
    }
}
