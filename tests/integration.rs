//! Integration tests for the Claim-0 vigil-esa crate (no kernel/eBPF/AWS).

use std::sync::Arc;
use vigil_esa::agents::ebpf::EbpfPolicyEngine;
use vigil_esa::core::cloud::CloudShield;
use vigil_esa::core::vulnerability::scanner::VulnerabilityScanner;
use vigil_esa::modules::hsm::HsmCryptoEngine;
use vigil_esa::modules::password_audit::cracker::PasswordAuditor;
use vigil_esa::core::network::mitm::arp_spoof::MitmSimulator;
use vigil_esa::{SecurityConfig, SecurityLoop};
use std::net::{IpAddr, Ipv4Addr};

#[tokio::test]
async fn security_loop_once_from_repo_config() {
    let cfg = SecurityConfig::load("config/security.toml").expect("config/security.toml");
    let loop_ = SecurityLoop::from_config(cfg);
    let beat = loop_.tick_once(1).await;
    assert_eq!(beat.tick, 1);
    assert!(beat.network_status.contains("ok"));
    assert!(beat.cloud_status.contains("ok"));
}

#[tokio::test]
async fn mock_ebpf_policy_without_bytecode_file() {
    let mut engine = EbpfPolicyEngine::load(b"not-real-bpf").unwrap();
    engine
        .apply_policy(Ipv4Addr::new(10, 0, 0, 1), Ipv4Addr::new(10, 0, 0, 2))
        .unwrap();
    assert_eq!(engine.policy_count(), 1);
}

#[tokio::test]
async fn offensive_stubs_refuse() {
    let hsm = Arc::new(HsmCryptoEngine::new_local("local_mock", "k"));
    let auditor = PasswordAuditor::new(hsm);
    assert!(auditor.crack(vec!["abc".into()]).await.is_empty());

    let sim = MitmSimulator::new();
    let report = sim.simulate(IpAddr::V4(Ipv4Addr::LOCALHOST)).await;
    assert_eq!(report.intercepted_packets, 0);
    assert!(report.message.contains("NOT FOR USE"));
}

#[tokio::test]
async fn vulnerability_scan_is_synthetic() {
    let hsm = Arc::new(HsmCryptoEngine::new_local("local_mock", "k"));
    let shield = CloudShield::new(hsm, "us-west-2".into(), "demo".into(), 1);
    let scanner = VulnerabilityScanner::new(&shield);
    let vulns = scanner.scan_cloud().await;
    assert_eq!(vulns.len(), 1);
}
