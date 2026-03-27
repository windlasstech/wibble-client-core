# PROJECT KNOWLEDGE BASE

**Project:** wibble-client-core  
**Stack:** Rust (edition 2024), Cargo workspace  
**Purpose:** Client-side MLS/OpenMLS security and protocol core for Wibble ecosystem

---

## OVERVIEW

Phase-0 bootstrap scaffold implementing RFC 9420 MLS protocol concepts. Three-layer architecture: Domain contracts → OpenMLS engine → Public facade. Unsafe code forbidden. Single-engine strategy (OpenMLS only).

---

## STRUCTURE

```
.
├── Cargo.toml                  # Workspace definition (3 members)
├── crates/
│   ├── wibble-core-domain/     # Layer 0: Domain contracts
│   ├── wibble-openmls-engine/  # Layer 1: OpenMLS integration
│   └── wibble-client-core/     # Layer 2: Public facade
├── docs/
│   ├── decisions/              # ADRs (MADR format)
│   └── architecture/           # Architecture docs
├── specs/
│   └── compatibility-matrix.yaml
└── .github/workflows/core-ci.yml
```

---

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Domain types | `crates/wibble-core-domain/src/lib.rs` | `GroupId`, `Epoch`, `GroupChangeIntent`, `CoreError` |
| Engine impl | `crates/wibble-openmls-engine/src/lib.rs` | `validate_change_intent`, `apply_change_intent` hooks |
| Public API | `crates/wibble-client-core/src/lib.rs` | `ClientCore` facade, re-exports domain types |
| Architecture decisions | `docs/decisions/ADR-*.md` | MADR 4.0.0 format |
| Protocol mapping | `docs/architecture/openmls-single-engine.md` | Layering and trust boundaries |
| RFC checklist | `docs/architecture/mls-protocol-checklist.md` | RFC 9420 compliance tracking |
| Compatibility spec | `specs/compatibility-matrix.yaml` | OpenMLS version, ciphersuites, interop |

---

## CODE MAP

| Symbol | Type | Crate | Role |
|--------|------|-------|------|
| `ClientCore` | struct | wibble-client-core | Public facade, owns engine |
| `OpenMlsEngine` | struct | wibble-openmls-engine | OpenMLS integration boundary |
| `GroupId` | struct | wibble-core-domain | MLS group identifier |
| `Epoch` | struct | wibble-core-domain | MLS epoch counter |
| `GroupChangeIntent` | struct | wibble-core-domain | Proposal/commit intent |
| `CoreError` | enum | wibble-core-domain | Domain error type |
| `ProposalKind` | enum | wibble-core-domain | Add/Update/Remove |

---

## CONVENTIONS

### Code
- `#![forbid(unsafe_code)]` mandatory in all crates
- Workspace-level metadata: version, edition, license in root `Cargo.toml`
- No stdlib extensions (no `tokio`, `serde` in Phase 0)
- Domain crate has **zero dependencies** (pure contracts)

### Documentation
- ADRs follow [MADR 4.0.0](https://adr.github.io/madr/)
- Architecture docs describe trust boundaries explicitly
- Comments cite RFC 9420 sections where applicable
- **Date notation: Use Holocene calendar (HE)** — add 10000 to Gregorian year (e.g., 2026 CE → 12026 HE)

### CI/CD
- `RUSTFLAGS: "-Dwarnings"` — warnings treated as errors
- `cargo fmt`, `clippy`, `check`, `test` — all workspace
- Runs on: push to `main`/`feat/**`, all PRs

---

## ANTI-PATTERNS (THIS PROJECT)

### Forbidden
- `unsafe` code — blocked at crate level
- Backend/infra/ops content in public repo — per ADR-0001
- Operational runbooks or secrets — governance violation
- Multi-engine abstraction in Phase 0/1 — out of scope per ADR-0002

### Current Limitations (Phase 0)
- Domain structs are **placeholder contracts**, not wire-level types
- OpenMLS integration stubbed (`NotImplemented` errors)
- `apply_change_intent` returns `Err` pending Phase 1

---

## COMMANDS

```bash
# Check workspace
cargo check --workspace

# Run tests
cargo test --workspace

# Linting (CI-enforced)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features

# Build
cargo build --workspace
```

---

## NOTES

- **Phase 1** will wire real OpenMLS types; current domain contracts will map
- **Trust boundary**: DS-delivered input is untrusted — validate before apply
- **Dependency direction**: domain → (none), engine → domain, client → domain+engine
- **Ciphersuite**: MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 (planned)
- Rust toolchain pinned to `1.94` via `rust-toolchain.toml`
