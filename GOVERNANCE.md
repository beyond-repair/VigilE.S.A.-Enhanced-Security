# Governance binding

This repository is bound to [beyond-repair/ADL-Governance](https://github.com/beyond-repair/ADL-Governance).

- Lifecycle state: **ARCHIVED** (documentary) with Claim-0 runnable sketch. GitHub `archived` flag is not set (operator-only).
- Sweep lock: **Sweep-193 / PASS-2026-10-02-193**. Pre-head `7221fb56c858c0b59120073c489125d737f83ec1`.
- Do not promote to ACTIVE without: real integrations (not mocks), non-offensive scope review, measured security evidence, and a green product gate.
- Do not expand offensive module bodies (`src/core/network/mitm/`, `src/modules/password_audit/`) into attack tools — keep refusal stubs.
- `.github/workflows/security_pipeline.yml` remains operator-owned and was not edited. Latest observed run 36863696288 concluded **failure**.
- `.github/workflows/claim0-tests.yml` is the sketch test gate (`cargo test --locked`). Green there is not a security-product claim.
- Duplicate `README .md` left in place (history-preserving).
