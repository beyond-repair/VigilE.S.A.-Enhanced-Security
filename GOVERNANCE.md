# Governance binding

This repository is bound to [beyond-repair/ADL-Governance](https://github.com/beyond-repair/ADL-Governance).

- Lifecycle state: **RESEARCH** (registry also lists freeze/archive candidate).
- Do not promote to ACTIVE without: Cargo manifest, tests, non-offensive scope, CI that runs `cargo test`, SECURITY.md review.
- Do not expand offensive module bodies (`src/core/network/mitm/`, `src/modules/password_audit/`).
- Operator actions (GitHub archive, tag, delete duplicate `README .md`, fix/disable failing SAST workflow) live in ADL-Governance `docs/OPERATOR_QUEUE.md`.
- Last governed sweep: Sweep-081 (2026-09-06).
