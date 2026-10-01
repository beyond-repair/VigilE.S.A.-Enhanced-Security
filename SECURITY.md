# Security policy (claim-capped)

## Scope

This repo is a Claim-0 architecture sketch with a local mock demo. Treat network, crypto, and audit modules as **non-operational for real protection**.

## Do not use for

- Offensive network operations (ARP spoof / MITM paths are refusal stubs).
- Password-cracking or credential attacks (`password_audit/cracker` returns empty).
- Production key management (HSM/KMS values in config are placeholders for local mocks).

## Reporting

Open a GitHub issue on this repository or on ADL-Governance. No bounty program is claimed.
