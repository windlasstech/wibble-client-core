---
status: "accepted"
date: 12026-03-28
decision-makers: "Yunseo Kim <yunseokim@windlasstech.com>"
informed: "Client platform maintainers"
---

# ADR-0002: OpenMLS Single-Engine Strategy

## Context and Problem Statement

Early planning included a multi-engine adapter direction, but the current phase focuses on fast bootstrap and conformance alignment. Expanding engine scope now would rapidly increase implementation, testing, and operational complexity.

## Decision Drivers

- Delivery speed for Phase 0/1
- Maturity of OpenMLS documentation and test assets
- Ability to control CI and verification complexity
- Lockstep alignment with conformance requirements

## Considered Options

- Option A: Develop with OpenMLS as the single engine.
- Option B: Implement OpenMLS + mls-rs as a simultaneous multi-engine adapter strategy.
- Option C: Switch to mls-rs as the single engine.

## Decision Outcome

Chosen option: "Option A", because it achieves current goals (bootstrap, conformance, and public-boundary compliance) at the lowest complexity.

### Consequences

- Good, because code/CI/test paths stay simpler and initial delivery is faster.
- Good, because OpenMLS references and documentation are immediately usable.
- Bad, because short-term multi-engine comparison data will not be produced.

### Confirmation

- Prioritize OpenMLS integration in `wibble-openmls-engine`.
- Verify that plans and CI assumptions remain single-engine consistent.

## Pros and Cons of the Options

### Option A: OpenMLS single engine

- Good, because implementation and verification scope stays controllable.
- Good, because docs and onboarding can align around one path.
- Bad, because engine-replacement experiments are deferred.

### Option B: Multi-engine in parallel

- Good, because replacement optionality data can be gathered earlier.
- Bad, because current-phase implementation and verification costs are too high.
- Bad, because API boundaries and test matrices become complex too quickly.

### Option C: mls-rs single engine

- Good, because FFI-friendly experiments can start quickly.
- Bad, because it diverges from current plan/docs/code direction.
- Bad, because alignment cost versus OpenMLS-based assets increases.

## More Information

- OpenMLS repository: `https://github.com/openmls/openmls`
- OpenMLS book: `https://book.openmls.tech/`
- RFC 9420: `https://github.com/mlswg/mls-protocol/blob/main/rfc9420.md`
