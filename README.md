# Wibble Client Core

![GitHub License](https://img.shields.io/github/license/windlasstech/wibble-client-core)
[![SemVer Versioning](https://img.shields.io/badge/version_scheme-SemVer-0097a7)](https://semver.org/)
[![GitHub issues](https://img.shields.io/badge/issue_tracking-GitHub-blue.svg)](https://github.com/windlasstech/wibble-client-core/issues)
[![core-ci](https://github.com/windlasstech/wibble-client-core/actions/workflows/core-ci.yml/badge.svg)](https://github.com/windlasstech/wibble-client-core/actions/workflows/core-ci.yml)
[![CodeQL](https://github.com/windlasstech/wibble-client-core/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/windlasstech/wibble-client-core/actions/workflows/github-code-scanning/codeql)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/windlasstech/wibble-client-core/badge)](https://scorecard.dev/viewer/?uri=github.com/windlasstech/wibble-client-core)

`wibble-client-core` is the public client-side security and protocol core for the Wibble ecosystem. It covers MLS/OpenMLS-based security primitives, message canonicalization, and cryptographic hashing utilities.
Current bootstrap uses an OpenMLS single-engine architecture with placeholder contracts for phase-0.

- **Core modules**: Security core, protocol logic, and shared utilities
- **Responsibility**: MLS/OpenMLS-based security primitives, canonicalization, and hashing
- **Design**: Platform-agnostic core
- **Note**: Tech stack to be determined per protocol-spec requirements

## Scope

- Client-side security/protocol core only
- RFC 9420 lifecycle concepts mapped as scaffold placeholders (phase-1 wires real OpenMLS types)
- No backend implementation details, runbooks, or secrets

## Bootstrap Architecture

Phase-0 scaffold is organized as a Rust workspace:

```
crates/
  wibble-core-domain/
  wibble-openmls-engine/
  wibble-client-core/
docs/
  adr/
  architecture/
specs/
  compatibility-matrix.yaml
```

Architecture docs:
- `docs/architecture/openmls-single-engine.md`
- `docs/architecture/mls-protocol-checklist.md`

Important:
- Current domain structs represent bootstrap intent types, not full RFC/OpenMLS wire-level models.
- OpenMLS integration and provider/storage wiring are phase-1 tasks.

## References

- MLS protocol (RFC 9420): https://github.com/mlswg/mls-protocol/blob/main/rfc9420.md
- MLS architecture draft: https://github.com/mlswg/mls-architecture/blob/main/draft-ietf-mls-architecture.md
- OpenMLS repository: https://github.com/openmls/openmls
- OpenMLS book: https://book.openmls.tech/
- Implementation inspiration: https://github.com/signalapp/libsignal/

## Build and Test

```bash
cargo check --workspace
cargo test --workspace
```

## Contributing

- [CONTRIBUTING.md](https://github.com/windlasstech/.github/blob/main/CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](https://github.com/windlasstech/.github/blob/main/CODE_OF_CONDUCT.md)
- [SECURITY.md](https://github.com/windlasstech/.github/blob/main/SECURITY.md)
- SUPPORT.md: coming soon

## Security

Security and privacy considerations follow the governance framework. Review the [SECURITY.md](https://github.com/windlasstech/.github/blob/main/SECURITY.md) for vulnerability reporting procedures.

## License

[Apache-2.0](./LICENSE)
