---
status: "accepted"
date: 12026-03-28
decision-makers: "Yunseo Kim <yunseokim@windlasstech.com>"
informed: "Client platform maintainers"
---

# ADR-0001: Public Repository Boundary and Non-Goals

## Context and Problem Statement

`wibble-client-core` is a public repository, so its code and docs are visible to external contributors and users. If operational details or server implementation specifics enter this repository, governance violations and security risks follow.

## Decision Drivers

- Compliance with the public-repo boundary defined in `GOVERNANCE_v1.md`
- Prevent exposure of security-sensitive information
- Keep repository responsibility focused on protocol core concerns

## Considered Options

- Option A: Include only client security/protocol core concerns and explicitly define non-goals.
- Option B: Include client core artifacts plus operational/backend reference documents.

## Decision Outcome

Chosen option: "Option A", because it directly minimizes exposure risk in a public repository and aligns with governance requirements.

### Consequences

- Good, because repository responsibilities and review criteria become clearer.
- Good, because leakage risk for backend/runbook/secret information is structurally reduced.
- Bad, because some internal team context must stay in separate private repositories.

### Confirmation

- Apply a public-boundary checklist in PR reviews.
- Enforce forbidden-keyword and sensitive-content scans in CI.

## Pros and Cons of the Options

### Option A: Public boundary focused on core only

- Good, because it matches governance and security requirements.
- Good, because repository purpose is obvious to external contributors.
- Bad, because links to internal operational context may be weaker.

### Option B: Include operational/backend docs

- Good, because internal onboarding could be easier.
- Bad, because public-repository boundary violations become likely.
- Bad, because security/policy review overhead increases.

## More Information

- Governance source: `https://github.com/windlasstech/internal-docs/blob/main/GOVERNANCE_v1.md`
