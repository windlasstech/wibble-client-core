---
status: "accepted"
date: 12026-03-28
decision-makers: "Yunseo Kim <yunseokim@windlasstech.com>"
informed: "Client platform maintainers, Security team"
---

# ADR-0003: SLSA Level 3 Release Pipeline

## Context and Problem Statement

`wibble-client-core` is a security-critical Rust library implementing MLS (Messaging Layer Security) protocol primitives. As part of the Wibble ecosystem, it requires strong supply chain security guarantees. Currently, the repository lacks:

- Build provenance documentation
- Signed artifact attestations
- Hermetic build isolation
- SBOM generation

Without these, we cannot provide verifiable guarantees about artifact integrity or build reproducibility.

## Decision Drivers

- Compliance with SLSA (Supply-chain Levels for Software Artifacts) v1.2 Build L3
- Protection against supply chain attacks (tampering, unauthorized modifications)
- Downstream consumer trust and verification capability
- Alignment with OpenSSF best practices

## Considered Options

### Option A: Adopt SLSA Level 3 with full GitHub Actions integration

Use GitHub's native artifact attestations, reusable workflows, and slsa-github-generator for provenance.

### Option B: Use external signing service (Sigstore/cosign directly)

Implement custom signing with Sigstore/cosign without GitHub's attestation infrastructure.

### Option C: Maintain status quo (SLSA Level 1 equivalent)

Continue with basic CI without provenance or attestation generation.

## Decision Outcome

Chosen option: **Option A** - Adopt SLSA Level 3 with full GitHub Actions integration.

### Rationale

- GitHub Actions provides native SLSA Build L3 builder capabilities
- `actions/attest` integrates seamlessly with GitHub's attestation API
- `slsa-github-generator` provides unforgeable provenance meeting SLSA Build L3 requirements
- Aligns with existing GitHub-centric workflow (Scorecard, CodeQL, etc.)

### Consequences

- Good, because artifacts have verifiable provenance cryptographically bound to the build
- Good, because downstream consumers can verify integrity using standard tools (`gh attestation verify`, `slsa-verifier`)
- Good, because SBOM generation provides dependency transparency
- Bad, because releases require tag-based triggers (cannot be built arbitrarily)
- Bad, because `slsa-github-generator` must be referenced by semver tag (exception to SHA-pin policy)

### Release Subjects

Primary release artifacts are `*.crate` files (Cargo packaged crates):

- `wibble-core-domain-0.1.0.crate`
- `wibble-openmls-engine-0.1.0.crate`
- `wibble-client-core-0.1.0.crate`

These are the stable, verifiable units distributed to consumers.

## Architecture

### Trusted Build Boundary

The reusable workflow `.github/workflows/reusable-rust-release-build.yml` is the **only** authorized build path. It:

1. Enforces `--locked` builds for reproducibility
2. Generates SBOM via `anchore/sbom-action`
3. Creates attestations via `actions/attest`
4. Outputs base64-encoded checksums for SLSA generator

### SLSA Generator Integration

Provenance generation uses `slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@v2.1.0`.

**IMPORTANT**: This is the **approved exception** to the repository's SHA-pin policy. The generator MUST be referenced by semver tag (`@v2.1.0`) for verification compatibility with `slsa-verifier`.

### Workflow Relationships

```
release.yml (orchestrator)
├── guard job (preflight checks)
├── build job (calls reusable-rust-release-build.yml)
├── provenance job (calls slsa-github-generator)
└── publish job (creates GitHub Release)
```

### Permissions Model

| Job | Required Permissions |
|-----|---------------------|
| guard | `contents: read` |
| build | `contents: read`, `id-token: write`, `attestations: write` |
| provenance | `actions: read`, `id-token: write`, `contents: write` |
| publish | `contents: write` |

## Verification

Consumers can verify releases using:

```bash
# Verify GitHub attestation
gh attestation verify wibble-client-core-0.1.0.crate \
  -R windlasstech/wibble-client-core

# Verify SLSA provenance
slsa-verifier verify-artifact wibble-client-core-0.1.0.crate \
  --provenance-path wibble-client-core.intoto.jsonl \
  --source-uri github.com/windlasstech/wibble-client-core \
  --source-tag v0.1.0
```

## Compliance Notes

### SLSA Build L3 Requirements (v1.2)

| Requirement | Implementation |
|-------------|----------------|
| Provenance Generation | `slsa-github-generator` + `actions/attest` |
| Provenance Authentic | Sigstore signing via GitHub attestation API |
| Provenance Unforgeable | `slsa-github-generator` trusted control plane |
| Isolation (Hosted) | GitHub Actions hosted build platform |
| Isolation (Isolated) | Reusable workflow with ephemeral environments |

### Exceptions

1. **SHA-pin Policy Exception**: `slsa-github-generator` uses semver tag `@v2.1.0` instead of SHA.
   - Justification: GitHub Actions limitation; `slsa-verifier` requires tag reference
   - Risk Mitigation: Generator is maintained by OpenSSF and widely audited

## References

- SLSA v1.2 Specification: https://slsa.dev/spec/v1.2/build-track-basics
- SLSA v1.2 Build Requirements: https://slsa.dev/spec/v1.2/build-requirements
- slsa-github-generator: https://github.com/slsa-framework/slsa-github-generator
- GitHub Artifact Attestations: https://docs.github.com/en/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds
- Architecture Doc: `docs/architecture/slsa-level-3-release.md`
