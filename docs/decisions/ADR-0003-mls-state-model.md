---
status: "proposed"
date: 12026-03-28
decision-makers: "Yunseo Kim <yunseokim@windlasstech.com>"
informed: "Client platform maintainers"
---

# ADR-0003: MLS State Model Baseline

## Context and Problem Statement

RFC 9420 defines epoch-based state transitions and a proposal/commit/welcome flow. During bootstrap, introducing all wire-level structures immediately is less effective than first fixing core state-transition contracts for staged integration.

## Decision Drivers

- Represent key RFC 9420 state concepts early
- Keep initial model complexity low
- Preserve contract stability before deeper OpenMLS integration

## Considered Options

- Option A: Start with minimal state contracts (`GroupId`, `Epoch`, `GroupChangeIntent`, `CoreError`).
- Option B: Adopt full RFC/OpenMLS wire-level types from the start.

## Decision Outcome

Chosen option: "Option A", because phase-0 benefits from stable and simple domain contracts, with OpenMLS-native type integration deferred to phase-1.

### Consequences

- Good, because core state-transition meaning can be reviewed and agreed quickly.
- Good, because testable contract boundaries exist before full OpenMLS wiring.
- Bad, because mapping work is still required between initial contracts and final wire-level models.

### Confirmation

- Verify domain contracts exist in `wibble-core-domain`.
- Verify `wibble-openmls-engine` validate/apply hooks consume those contracts.

## Pros and Cons of the Options

### Option A: Minimal contract-first baseline

- Good, because bootstrap speed is higher.
- Good, because API boundaries can stabilize earlier.
- Bad, because phase-1 mapping work remains.

### Option B: Full wire-level adoption immediately

- Good, because some future refactoring might be reduced.
- Bad, because initial implementation and change cost increases significantly.
- Bad, because failed doc/code synchronization causes harder debugging.

## More Information

- RFC 9420: `https://github.com/mlswg/mls-protocol/blob/main/rfc9420.md`
- MLS architecture: `https://github.com/mlswg/mls-architecture/blob/main/draft-ietf-mls-architecture.md`
