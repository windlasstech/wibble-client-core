# wibble-client-core Architecture and Repository Bootstrap Plan (v1)

Date: 2026-03-28  
Target branch: `feat/repo-bootstrap-architecture`

## Decision Update (2026-03-28)

- This plan is fixed to an `OpenMLS single-engine` development strategy.
- Multi-engine adapter expansion is out of current scope and retained as reference research only.
- Architectural decisions are managed through ADR files under `docs/decisions/`.
- ADRs in `docs/decisions/` follow the MADR 4.0.0 template.

## Executive Summary

- `wibble-client-core` handles only the client-side security and protocol core within a public-repo boundary.
- The design baseline is RFC 9420 state evolution (epoch), proposal/commit/welcome flow, and AS/DS trust boundaries.
- The implementation engine is OpenMLS only.
- Phase 0/1/2 all target OpenMLS single-engine stabilization and conformance alignment.

## Source-Backed Findings

- Governance requires excluding backend details, operational runbooks, and secrets from public repositories. [S1]
- Protocol behavior changes require lockstep updates with `protocol-spec` and `conformance-tests`. [S1]
- MLS is an embeddable security layer with a trusted AS boundary and an untrusted DS boundary. [S2]
- RFC 9420 defines group/epoch state, KeyPackage/Proposal/Commit/Welcome flow, and ratchet tree/key schedule separation. [S3]
- OpenMLS provides strong public documentation and testing assets and best matches the current single-engine plan. [S4][S7]

## MLS Implementation Positioning (Current Plan)

Observation baseline: 2026-03-28, `implementation_list.md` plus public repository metadata [S4][S7]

- Adopted: `OpenMLS` (`openmls/openmls`)
- Non-adopted comparison references: `mls-rs`, `MLSpp`, `ts-mls`
- Purpose: current delivery is OpenMLS single-engine; other implementations are retained only for interoperability research context

## Recommended Architecture

### 1) Layered Structure

- Layer 0: Domain Contracts
  - `GroupId`, `Epoch`, `GroupChangeIntent`, `CoreError`
  - Protocol contract types and validation points
- Layer 1: OpenMLS Engine
  - OpenMLS integration boundary
  - Input validation and apply hooks
- Layer 2: Policy & Validation
  - Membership/authorization policy, commit acceptance policy
  - Guards against spec/conformance drift
- Layer 3: Serialization/Canonicalization
  - Wire canonicalization, hashing utilities, test-vector formats
- Layer 4: Public Facade / Bindings Surface
  - Public API via `wibble-client-core`
  - No direct exposure of internal crypto objects from platform bindings

### 2) Module Boundaries

- `wibble-core-domain` must not depend on OpenMLS implementation details.
- `wibble-openmls-engine` handles OpenMLS integration and state transition execution.
- `wibble-client-core` provides the external entry point and encapsulates engine calls.

### 3) Trust Boundaries

- Trusted zone: local key/state storage, signature keys, epoch secrets
- Semi-trusted zone: AS verification-result cache
- Untrusted zone: DS inputs and network-delivered messages
- Rule: DS-originated data is always validated before state mutation. [S2][S3]

## Proposed Repo Structure

```text
wibble-client-core/
  docs/
    decisions/
      ADR-0001-repo-boundary.md
      ADR-0002-openmls-single-engine.md
      ADR-0003-mls-state-model.md
    architecture/
      openmls-single-engine.md
      mls-protocol-checklist.md
  specs/
    compatibility-matrix.yaml
  crates/
    wibble-core-domain/
    wibble-openmls-engine/
    wibble-client-core/
  .github/
    workflows/
      core-ci.yml
  README.md
  LICENSE
```

Structure principles:
- `backend`, `infra`, `ops`, `runbooks`, and `prod` artifacts are out of scope. [S1]
- Align docs/code/CI to an OpenMLS single-engine baseline.
- Manage conformance baseline through `specs/compatibility-matrix.yaml` plus external conformance sources.

## Build/Test/CI Strategy

- Core lane (required)
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo check --workspace`
  - `cargo test --workspace`
- Compatibility lane (after Phase 1)
  - Validate `specs/compatibility-matrix.yaml`
  - Run conformance subset vectors
- Docs/Governance lane
  - MADR format checks
  - Public-boundary keyword scans

## Decision Log / ADR Backlog

Recommended order:

1. ADR-0001: Public Boundary & Non-Goals
2. ADR-0002: OpenMLS Single-Engine Strategy
3. ADR-0003: MLS State Model Baseline

Add further ADRs only as needed, using the same MADR format.

## Phased Execution Plan

### Phase 0 - Foundation (1-2 weeks)

- Goal: repository skeleton, MADR-aligned docs, and minimal CI baseline
- Deliverables:
  - Align 3 ADRs in `docs/decisions/` to MADR
  - Keep Rust workspace skeleton intact
  - Enable core CI lane
  - Initialize `specs/compatibility-matrix.yaml`

### Phase 1 - OpenMLS Core Bootstrap (2-4 weeks)

- Goal: secure a minimum viable OpenMLS single-engine execution path
- Deliverables:
  - Start real OpenMLS integration in `wibble-openmls-engine`
  - Connect domain contracts to OpenMLS state transitions
  - Automate conformance subset execution

### Phase 2 - OpenMLS Hardening (3-6 weeks)

- Goal: improve single-engine stability and interoperability regression coverage
- Deliverables:
  - Expand interoperability/regression tests
  - Document API stability criteria
  - Finalize release criteria and lockstep operating process

## Open Questions / Risks

- Ongoing cost of tracking OpenMLS API changes
- Timing risk for RFC extension adoption (extensions/PQ) [S6]
- FFI/binding complexity around memory ownership and error mapping
- Risk of leaking internal operational information into public docs [S1]

## Recommendation

- Fix strategy to `OpenMLS single-engine` now.
- Keep multi-engine comparison/replacement out of current execution scope and only as research context.
- Prioritize OpenMLS integration correctness, conformance alignment, and public-boundary compliance.

---

## Sources

- [S1] `GOVERNANCE_v1.md` (retrieved via gh CLI): `https://github.com/windlasstech/internal-docs/blob/main/GOVERNANCE_v1.md`
- [S2] MLS Architecture: `https://github.com/mlswg/mls-architecture/blob/main/draft-ietf-mls-architecture.md`
- [S3] MLS Protocol RFC 9420: `https://github.com/mlswg/mls-protocol/blob/main/rfc9420.md`
- [S4] MLS implementation list: `https://raw.githubusercontent.com/mlswg/mls-implementations/refs/heads/main/implementation_list.md`
- [S5] MLS WG hub: `https://github.com/mlswg`
- [S6] IETF MLS WG documents: `https://datatracker.ietf.org/wg/mls/documents/`
- [S7] Open source implementation repositories (including `openmls/openmls`)
