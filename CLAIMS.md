# Claims register — VigilE.S.A.-Enhanced-Security

Policy: ADL-Governance `docs/CLAIM_VALIDATION.md`.

| Claim | Allowed level | Evidence |
|-------|---------------|----------|
| Open-source security platform protecting networks/cloud/endpoints | 0 | Architecture sketch + mock demo only |
| Zero Trust / confidential computing / HSM / AI detection | 0 | Module names + in-memory mocks; no measurements |
| eBPF filtering | 0 | `EbpfPolicyEngine` HashMap mock; no kernel program |
| SGX/SEV enclaves | 0 | `SgxEnclave` local seal mock |
| Wasm security plugins | 0 | Echo mock; no wasmtime |
| Compiles with `cargo build` | 0 (verified for sketch) | Local `cargo test --locked` Sweep-193: 15 unit + 4 integration passed |
| `cargo test` passes without AWS/eBPF | 0 (verified for sketch) | Same local run; remote `claim0-tests` conclusion not claimed until Actions completes |
| Demo CLI `demo` / `once` | 0 (verified for sketch) | `vigil-esa` binary |
| Security Pipeline is product CI green | not claimed | Workflow untouched; run 36863696288 failure |
| `claim0-tests` is a security-efficacy gate | not claimed | Sketch compile/test only |
| Offensive ARP/MITM/password cracking | **refused** | Safe no-op stubs; empty/refusal returns |

Software readiness is tracked separately from security-efficacy claims. Claim-0 runnable sketch is not engineering validation (Level 5).
