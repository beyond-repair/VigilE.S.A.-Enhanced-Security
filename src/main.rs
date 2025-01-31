//! VigilE.S.A. - Enhanced Security Architecture
//!
//! This is the main entry point for the VigilE.S.A. system.
//! It initializes all core components and starts the security loop.

use core::network::NetworkMonitor;
use core::cloud::CloudShield;
use modules::hsm::HsmCryptoEngine;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize HSM for cryptographic operations
    let hsm = Arc::new(HsmCryptoEngine::new("arn:aws:kms:...").await);

    // Start network monitoring
    let network_monitor = NetworkMonitor::new(hsm.clone());
    network_monitor.start().await;

    // Initialize cloud security
    let cloud_shield = CloudShield::new(hsm);
    cloud_shield.enable_protection().await;

    // Main security loop
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
