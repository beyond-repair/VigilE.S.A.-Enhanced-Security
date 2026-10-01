# Governance binding

This repository is bound to [beyond-repair/ADL-Governance](https://github.com/beyond-repair/ADL-Governance).

- Lifecycle state: **ARCHIVE** with Claim-0 runnable sketch.
- Do not promote to ACTIVE without: real integrations (not mocks), non-offensive scope review, measured security evidence, CI that runs `cargo test` as product gate.
- Do not expand offensive module bodies (`src/core/network/mitm/`, `src/modules/password_audit/`) into attack tools — keep refusal stubs.
- `.github/workflows/security_pipeline.yml` is operator-owned; Finish repair leaves it unchanged.
- Duplicate `README .md` left in place (history-preserving).
