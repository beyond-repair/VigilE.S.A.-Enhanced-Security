# VigilE.S.A. Architecture (Claim-0 sketch)

## Overview

Historical design intent:

- Zero Trust Networking
- Confidential Computing
- Hardware-backed Cryptography
- AI-driven Threat Detection

**Claim-0 reality:** those names map to **local in-memory mocks** suitable for demonstrating module structure and a security-loop heartbeat. No real eBPF, SGX, HSM, or multi-cloud clients are required or invoked on the default path.

## Components (as implemented in the sketch)

1. **Network security (`src/core/network`)**
   - `NetworkMonitor`: config-driven mock heartbeat (no packet capture).
   - `mitm/arp_spoof`: filename preserved; **refusal stub** (not for use).
2. **Cloud security (`src/core/cloud`)**
   - `CloudShield`: synthetic resource list + heartbeat (no AWS/GCP SDK).
3. **HSM (`src/modules/hsm`)**
   - `HsmCryptoEngine`: local SHA-256 fingerprint "sign" (no KMS).
4. **Enclave / Wasm / eBPF agents**
   - In-process mocks only (`SgxEnclave`, `WasmPolicyEngine`, `EbpfPolicyEngine`).
5. **Incident / vulnerability**
   - Mock forensic hash + synthetic CIS-style findings.

## Demo control flow

```
config/security.toml → SecurityConfig → SecurityLoop
       → NetworkMonitor.heartbeat
       → CloudShield.heartbeat
       → HsmCryptoEngine.status
       → CLI prints ticks (`demo` / `once` / `run`)
```
