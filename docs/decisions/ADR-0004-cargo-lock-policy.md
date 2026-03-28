---
status: "accepted"
date: 12026-03-28
decision-makers: "Yunseo Kim <yunseokim@windlasstech.com>"
informed: "Client platform maintainers, Security team"
---

# ADR-0004: Committing Cargo.lock for Reproducible Builds

## Context and Problem Statement

When implementing SLSA Build L3 compliance for `wibble-client-core`, we encountered a fundamental question: should a Rust library commit its `Cargo.lock` file to version control?

This question arose because:
- The CI workflow uses `--locked` flag for reproducible builds (SLSA L3 requirement)
- Without `Cargo.lock` committed, `--locked` fails because the lock file doesn't exist in CI
- Traditional Rust guidance suggested libraries should NOT commit `Cargo.lock`
- But security-critical libraries like `rustls` and `ring` DO commit `Cargo.lock`

We needed to determine the best approach for a security-critical MLS (Messaging Layer Security) library.

## Decision Drivers

1. **SLSA Build L3 Compliance**: Requires reproducible builds with provenance
2. **Supply Chain Security**: Prevent dependency confusion and unauthorized changes
3. **Industry Practices**: What do other security-critical Rust libraries do?
4. **Rust Ecosystem Evolution**: Cargo team changed guidance in August 2023
5. **Auditability**: Security audits require knowing exact dependency versions

## Investigation

### Rust Official Guidance Evolution

**Old Guidance (pre-2023)**:
- Binaries: Commit `Cargo.lock`
- Libraries: Do NOT commit `Cargo.lock`
- Rationale: Libraries should test against latest dependencies

**New Guidance (August 2023)**:
> "We now recommend people **do what is best for their project**... We suggest committing `Cargo.lock` as a starting point."

The Cargo team explicitly recognized that the old binary/library distinction was too rigid. The new guidance emphasizes project-specific needs over categorical rules.

### Security Library Practices

We investigated major Rust security/cryptography libraries:

| Library | Domain | Commits Cargo.lock | Uses `--locked` | Rationale |
|---------|--------|-------------------|-----------------|-----------|
| **rustls** | TLS | ✅ Yes | ✅ Yes | Security-first, reproducible builds |
| **ring** | Cryptography | ✅ Yes | ✅ Yes | Chrome/Firefox dependency |
| **curve25519-dalek** | Signatures | ✅ Yes | ✅ Yes | Signal/WhatsApp usage |
| **openmls** | MLS Protocol | ❌ No | ❌ No | Tests against latest deps |
| **libsodium-rs** | Cryptography | ❌ No | ❌ No | Traditional library approach |

### Key Findings

1. **Premium security libraries (rustls, ring, dalek) commit Cargo.lock**
   - These are the most widely-audited, security-critical Rust libraries
   - They prioritize reproducible builds and auditability
   - All use `--locked` in CI to guarantee exact dependency versions

2. **MLS-specific library (openmls) does NOT commit Cargo.lock**
   - Same protocol domain (RFC 9420 MLS)
   - Prioritizes catching dependency compatibility issues early
   - Traditional library-focused approach

3. **The split is about priorities, not correctness**
   - Commit approach: Security auditability, reproducible builds
   - Exclude approach: Latest dependency testing, ecosystem compatibility

## Considered Options

### Option A: Commit Cargo.lock (rustls model)

**Implementation**:
- Remove `Cargo.lock` from `.gitignore`
- Keep `--locked` flag in all CI commands
- Dependabot/Renovate for dependency updates

**Pros**:
- ✅ Meets SLSA Build L3 "reproducible builds" requirement
- ✅ Prevents dependency confusion attacks
- ✅ Enables exact dependency audit for security reviews
- ✅ Aligns with rustls/ring/curve25519-dalek practices
- ✅ Supports `cargo bisect` for regression hunting

**Cons**:
- ❌ Larger PR volume from Dependabot
- ❌ May miss dependency compatibility issues until Dependabot PR
- ❌ Deviates from traditional library convention

### Option B: Exclude Cargo.lock (openmls model)

**Implementation**:
- Keep `Cargo.lock` in `.gitignore`
- Remove `--locked` flag from CI
- Generate lock file in CI with `cargo generate-lockfile`

**Pros**:
- ✅ Catches dependency issues immediately
- ✅ Smaller repository size
- ✅ Aligns with openmls (same MLS domain)
- ✅ Follows old Rust guidance

**Cons**:
- ❌ CI builds may differ from local builds
- ❌ Cannot guarantee reproducible builds for SLSA L3
- ❌ Security audits cannot verify exact dependency tree
- ❌ Build failures when dependencies are yanked

### Option C: Hybrid (commit lock, test latest separately)

**Implementation**:
- Commit `Cargo.lock` for reproducible builds
- Add scheduled CI job that runs `cargo update` and tests

**Pros**:
- ✅ Best of both worlds
- ✅ Reproducible builds + latest dependency testing

**Cons**:
- ❌ More complex CI configuration
- ❌ Still requires solving the immediate `--locked` CI failure

## Decision Outcome

Chosen option: **Option A - Commit Cargo.lock** (rustls model)

### Rationale

For a **security-critical MLS implementation**, the benefits of reproducible builds and auditability outweigh the benefits of testing against latest dependencies.

**Key reasons**:

1. **SLSA Build L3 Alignment**: The specification explicitly requires reproducible builds with provenance. A committed lock file is the foundation of this requirement.

2. **Security Domain**: MLS is used for end-to-end encrypted messaging. A compromised dependency could leak messages or break encryption. Exact dependency tracking is essential.

3. **Industry Precedent**: rustls, ring, and curve25519-dalek are the gold standard for Rust cryptography. Their choice to commit lock files validates this approach for security libraries.

4. **Supply Chain Attack Prevention**: Recent attacks (SolarWinds, CodeCov, xz backdoor) demonstrate the importance of knowing exactly what code is in your dependency tree.

5. **Auditability**: Security auditors need to verify exact dependency versions. "Latest compatible version" is insufficient for a cryptographic audit.

6. **Rust Ecosystem Evolution**: The Cargo team's 2023 guidance change explicitly supports this decision: "do what is best for your project."

### Consequences

- **Good**: Reproducible builds enable SLSA Build L3 compliance
- **Good**: Exact dependency versions support security audits
- **Good**: CI builds match local builds exactly (with `--locked`)
- **Good**: Prevents build failures from yanked dependencies
- **Neutral**: Dependabot will create more PRs (managed with auto-merge for patches)
- **Neutral**: Larger repository size (minimal - lock file is ~20KB)

## Implementation

### Changes Made

1. **`.gitignore`**: Removed `Cargo.lock` entry, added comment referencing this ADR
2. **Cargo.lock**: Added to version control
3. **CI workflows**: Keep `--locked` flag (already implemented for SLSA L3)
4. **Dependabot**: Already configured for automatic dependency updates

### Verification

After implementation:
- CI builds with `--locked` will succeed
- `cargo build --locked` locally will use committed lock file
- Dependabot will propose dependency updates via PRs
- Security audits can reference exact dependency versions

## References

- Rust Cargo Team Guidance (Aug 2023): https://blog.rust-lang.org/2023/08/29/committing-lockfiles/
- Cargo FAQ: https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries
- SLSA Build L3 Requirements: https://slsa.dev/spec/v1.2/build-track-basics
- rustls repository: https://github.com/rustls/rustls (Cargo.lock committed)
- ring repository: https://github.com/briansmith/ring (Cargo.lock committed)
- openmls repository: https://github.com/openmls/openmls (Cargo.lock excluded)
- ADR-0003: SLSA Build L3 Release Pipeline
