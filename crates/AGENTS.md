# CRATES LAYER OVERVIEW

**Location:** `crates/` — Workspace member crates  
**Architecture:** Three-layer domain → engine → facade stack

---

## STRUCTURE

```
crates/
├── wibble-core-domain/         # Layer 0: Domain contracts
├── wibble-openmls-engine/      # Layer 1: OpenMLS integration
└── wibble-client-core/         # Layer 2: Public facade
```

---

## WHERE TO LOOK

| Task | Crate | File | Notes |
|------|-------|------|-------|
| Domain types | wibble-core-domain | `src/lib.rs` | Zero dependencies, pure contracts |
| Validation hooks | wibble-openmls-engine | `src/lib.rs` | `validate_change_intent()`, `apply_change_intent()` |
| Public API | wibble-client-core | `src/lib.rs` | `ClientCore` struct, re-exports |

---

## CONVENTIONS

### Crate Dependencies
```
wibble-core-domain ← (no deps)
wibble-openmls-engine ← wibble-core-domain
wibble-client-core ← wibble-core-domain + wibble-openmls-engine
```

### Each Crate
- Single `src/lib.rs` entry point
- `#![forbid(unsafe_code)]` at top
- `Cargo.toml`: uses `workspace = true` for shared metadata

---

## ANTI-PATTERNS

- **Never** add dependencies to `wibble-core-domain` — keep it pure
- **Never** expose OpenMLS internals through `wibble-client-core` facade
- **Never** let engine layer depend on facade layer (no circular deps)

---

## NOTES

- Phase 0: All crates are thin scaffolds with placeholder types
- Phase 1: Engine crate will gain real OpenMLS dependencies
- Domain contracts will map to OpenMLS types during Phase 1
