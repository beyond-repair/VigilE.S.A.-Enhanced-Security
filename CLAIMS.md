# Claims register — VigilE.S.A.-Enhanced-Security

Policy: ADL-Governance `docs/CLAIM_VALIDATION.md`.

| Claim in historical README | Allowed level | Evidence |
|----------------------------|---------------|----------|
| Open-source security platform protecting networks/cloud/endpoints | 0 | Tree + prose only |
| Zero Trust / confidential computing / HSM / AI detection | 0 | Module filenames; no measurements |
| eBPF filtering | 0 | `src/agents/ebpf/mod.rs` exists; no verified program |
| SGX/SEV enclaves | 0 | `src/modules/enclave/mod.rs` exists |
| Blockchain-backed logs | 0 | No ledger implementation found |
| Wasm security plugins | 0 | `src/agents/wasm/mod.rs` exists |
| Compiles with `cargo build --release` | falsified as written | **No Cargo.toml** |
| Security Pipeline is product CI green | falsified this cycle | Actions run 33992096428 `failure` |

Software readiness is tracked separately from physics/security-efficacy claims. Green SAST checkout ≠ engineering validation (Level 5).
Sweep-081 (2026-09-06): claim table unchanged except CI row.
