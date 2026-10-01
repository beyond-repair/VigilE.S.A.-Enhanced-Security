//! Incident response module — Claim-0 mock forensics / response.
//!
//! No SGX. Disk "analysis" hashes bytes in-process only.

use crate::modules::enclave::SgxEnclave;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// Demo forensic report (not a real investigation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForensicReport {
    pub hash_hex: String,
    pub indicators: Vec<String>,
}

pub struct ForensicAnalyzer {
    enclave: SgxEnclave,
}

impl ForensicAnalyzer {
    pub fn new() -> Self {
        Self {
            enclave: SgxEnclave::new().expect("local mock enclave"),
        }
    }

    /// Hash file contents via the mock enclave seal path (local only).
    pub fn analyze_disk(&self, image_path: PathBuf) -> Result<ForensicReport, String> {
        let raw_data = std::fs::read(&image_path).map_err(|e| e.to_string())?;
        let sealed = self.enclave.process(&raw_data).map_err(|e| e.to_string())?;
        let mut hasher = Sha256::new();
        hasher.update(&sealed);
        Ok(ForensicReport {
            hash_hex: hex::encode(hasher.finalize()),
            indicators: vec!["claim0-mock-indicator".into()],
        })
    }
}

impl Default for ForensicAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Severity for the mock response engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Low,
}

#[derive(Debug, Clone)]
pub struct Threat {
    pub severity: Severity,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseAction {
    IsolateNetwork,
    RotateCredentials,
    LogIncident,
}

pub struct ResponseEngine;

impl ResponseEngine {
    pub fn new() -> Self {
        Self
    }

    /// Decide a mock containment action (no real network isolation).
    pub async fn contain_threat(&self, threat: &Threat) -> ResponseAction {
        match threat.severity {
            Severity::Critical => ResponseAction::IsolateNetwork,
            Severity::High => ResponseAction::RotateCredentials,
            Severity::Low => ResponseAction::LogIncident,
        }
    }
}

impl Default for ResponseEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn analyze_disk_hashes_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "demo-image").unwrap();
        let analyzer = ForensicAnalyzer::new();
        let report = analyzer.analyze_disk(tmp.path().to_path_buf()).unwrap();
        assert_eq!(report.hash_hex.len(), 64);
        assert!(!report.indicators.is_empty());
    }

    #[tokio::test]
    async fn response_maps_severity() {
        let eng = ResponseEngine::new();
        let action = eng
            .contain_threat(&Threat {
                severity: Severity::Critical,
                name: "mock".into(),
            })
            .await;
        assert_eq!(action, ResponseAction::IsolateNetwork);
    }
}
