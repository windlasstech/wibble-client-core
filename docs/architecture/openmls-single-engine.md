# OpenMLS Single-Engine Bootstrap Architecture

## Scope

This document defines the phase-0 scaffold architecture for `wibble-client-core` using OpenMLS as the single engine.

## Layering

1. Domain (`wibble-core-domain`)
   - Protocol-facing core contracts: `GroupId`, `Epoch`, `GroupChangeIntent`.
2. Engine (`wibble-openmls-engine`)
   - OpenMLS integration boundary and commit validation/application hooks.
3. Facade (`wibble-client-core`)
   - Stable library surface for downstream client apps.

## Trust Boundary Baseline

- Trusted: local key/state handling.
- Untrusted: DS-delivered network input.
- Validation before apply is mandatory for incoming commit paths.

## Protocol Mapping

- Proposal/Commit/Welcome flow follows RFC 9420 semantics.
- AS/DS separation follows MLS architecture model.
- Phase-0 domain structs are intentionally simplified placeholder contracts.
- Phase-1 is the point where OpenMLS-native message/state/provider types become source of truth.

## References

- https://github.com/mlswg/mls-protocol/blob/main/rfc9420.md
- https://github.com/mlswg/mls-architecture/blob/main/draft-ietf-mls-architecture.md
- https://github.com/openmls/openmls
- https://book.openmls.tech/
- https://github.com/signalapp/libsignal/
