<div align="center">

[![Lifecycle](https://img.shields.io/badge/●_ARCHIVE-64748b?style=for-the-badge&labelColor=0f0f23)](https://github.com/beyond-repair/ADL-Governance)
[![Claim](https://img.shields.io/badge/Claim_0-22c55e?style=for-the-badge&labelColor=0f0f23)](https://github.com/beyond-repair/ADL-Governance/blob/main/docs/CLAIM_VALIDATION.md)
[![Governance](https://img.shields.io/badge/ADL--Governance-7c3aed?style=for-the-badge&labelColor=0f0f23)](https://github.com/beyond-repair/ADL-Governance)

```
LIFECYCLE   ARCHIVE
CLAIM       0
NOT CLAIMED product · profit · deployment · real ZTNA/eBPF/HSM
```

</div>

> **ARCHIVE / Claim-0 runnable sketch.** Historical security-architecture concept with a local mock demo. Not a production security product.

---

# VigilE.S.A. Enhanced Security — Claim-0 runnable sketch

**Classification:** ARCHIVE (research sketch kept runnable).  
**Claim level:** 0 — local mock security loop only.  
**Governing source:** [ADL-Governance](https://github.com/beyond-repair/ADL-Governance).

This repository is a **2025-era modular Rust security-architecture sketch** repaired into a **clean-clone-verified demo**. It is **not** a validated Zero Trust platform, not a production endpoint agent, and not an independently measured security product.

## What runs (Claim-0)

| Item | Status |
|------|--------|
| `Cargo.toml` binary crate `vigil-esa` | YES |
| `cargo build` / `cargo test` (std + common crates) | YES — no AWS/GCP/aya/eBPF required |
| Demo CLI: load `config/security.toml`, mock heartbeats, `--demo` / `--once` | YES |
| Mock `NetworkMonitor`, `CloudShield`, `HsmCryptoEngine` | in-memory only |
| Offensive-named paths (`arp_spoof`, `password_audit/cracker`) | **safe no-op stubs** — refuse; not on the demo path |
| Real eBPF / SGX / SEV / HSM / cloud protection | **NOT CLAIMED** |

## Quick start

```bash
git clone https://github.com/beyond-repair/VigilE.S.A.-Enhanced-Security.git
cd VigilE.S.A.-Enhanced-Security

cargo build
cargo test

# CI-friendly demo (reads config/security.toml, prints mock heartbeats, exits)
cargo run -- demo

# Single heartbeat
cargo run -- once

# Config summary
cargo run -- status
```

Optional continuous mock loop (Ctrl-C to exit cleanly):

```bash
cargo run -- run
```

Requires a recent stable Rust toolchain (`rustc` / `cargo`).

## What is not claimed

- Production-ready network or cloud protection.
- Hardware-backed cryptography or confidential-computing attestation.
- AI-driven threat detection efficacy.
- Authorization or ability to perform ARP spoofing, password cracking, or MITM.
- That `.github/workflows/security_pipeline.yml` is a green product test suite (left unchanged; SAST/cosign only).

See [CLAIMS.md](CLAIMS.md) and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Project layout (preserved identity)

```
config/security.toml     demo config (local mocks)
src/main.rs              clap CLI entry
src/lib.rs               SecurityLoop orchestration
src/core/                network / cloud / incident / vulnerability mocks
src/modules/             hsm + enclave mocks; password_audit refusal stub
src/agents/              ebpf + wasm in-memory mocks
tests/integration.rs     no kernel/cloud required
```

Duplicate filename `README .md` is left in place (history-preserving).

---

<div align="center">

**REWRITE · BUILD · TRANSCEND**

Governing source: [ADL-Governance](https://github.com/beyond-repair/ADL-Governance) · [Claim levels 0–5](https://github.com/beyond-repair/ADL-Governance/blob/main/docs/CLAIM_VALIDATION.md)

</div>
