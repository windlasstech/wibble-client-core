# MLS Protocol Scaffold Checklist

## Required Concepts (RFC 9420)

- Group and epoch state evolution.
- Proposal kinds: Add, Update, Remove.
- Commit application after proposal validation.
- Welcome flow for new member join.

## Trust and Service Model (MLS Architecture)

- Authentication service trust boundary.
- Delivery service untrusted boundary.
- Client-side verification before state mutation.

## OpenMLS Integration Checklist

- Define provider/storage strategy in phase 1.
- Start with MTI ciphersuite for interoperability in phase 1.
- Build conformance-vector ingestion in phase 1/2.
