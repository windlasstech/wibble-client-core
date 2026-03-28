# SLSA Build L3 Release Pipeline Architecture

**Date**: 12026-03-28 HE  
**Status**: Implemented  
**Reference**: ADR-0003  
**SLSA Spec**: v1.2

## Overview

This document describes the SLSA (Supply-chain Levels for Software Artifacts) Build L3 compliant release pipeline for `wibble-client-core`. The pipeline ensures that every release artifact has cryptographically verifiable provenance, documenting exactly how it was built and from what source.

### What is SLSA Build L3?

SLSA Build L3 (Level 3) provides the following guarantees per [SLSA v1.2 spec](https://slsa.dev/spec/v1.2/build-track-basics):

1. **Provenance is Unforgeable**: Build platform generates and signs provenance; signing keys are inaccessible to build steps
2. **Isolated**: Builds run in ephemeral environments; overlapping builds cannot influence each other
3. **Tamper Prevention**: Prevents tampering during the build by insider threats or compromised credentials
4. **Strong Confidence**: Provides strong evidence that the package was built from the official source and build process

## Artifact Inventory

Each release includes the following artifacts:

| Artifact | Description | Verification Method |
|----------|-------------|---------------------|
| `*.crate` | Cargo packaged crates (primary release subjects) | GitHub attestation + SLSA provenance |
| `subject.checksums.txt` | SHA256 checksums of all crates | Manual verification |
| `sha256SUMS` | Comprehensive checksum file | `sha256sum -c` |
| `wibble-client-core.spdx.json` | SPDX-formatted SBOM | GitHub attestation |
| `wibble-client-core.intoto.jsonl` | SLSA provenance attestation | `slsa-verifier` |

### Primary Release Subjects

The `*.crate` files are the primary release subjects:

- `wibble-core-domain-{version}.crate`
- `wibble-openmls-engine-{version}.crate`
- `wibble-client-core-{version}.crate`

These are the artifacts consumers download and verify.

## Workflow Architecture

### Release Pipeline Flow

```
┌─────────────────┐     ┌─────────────────────────┐     ┌──────────────────┐
│   Tag Push      │────▶│   release.yml           │────▶│   Guard Job      │
│   (v* pattern)  │     │   (Orchestrator)        │     │   (Preflight)    │
└─────────────────┘     └─────────────────────────┘     └──────────────────┘
                                                                  │
                                                                  ▼
                        ┌─────────────────────────┐     ┌──────────────────┐
                        │   Publish Job           │◀────│   Build Job      │
                        │   (GitHub Release)      │     │   (Reusable WF)  │
                        └─────────────────────────┘     └──────────────────┘
                                                                  │
                                                                  ▼
                        ┌─────────────────────────┐     ┌──────────────────┐
                        │   SLSA Provenance       │◀────│   Provenance Job │
                        │   (intoto.jsonl)        │     │   (Generator)    │
                        └─────────────────────────┘     └──────────────────┘
```

### Workflow Files

| Workflow | Purpose | Trigger |
|----------|---------|---------|
| `release.yml` | Orchestrates the entire release | Push tags `v*`, workflow_dispatch |
| `reusable-rust-release-build.yml` | Trusted build boundary | Called by release.yml |
| `verify-release.yml` | Post-release verification | workflow_run, manual |

### Trusted Build Boundary

The reusable workflow `reusable-rust-release-build.yml` is the **only** authorized build path. Key characteristics:

- **Hermetic**: Uses `--locked` flag for all cargo commands
- **Isolated**: Runs in hardened runner with egress audit
- **Attested**: Generates GitHub attestations for provenance and SBOM
- **Reproducible**: Locked dependencies ensure consistent builds

## Permissions Reference

### release.yml

| Job | Contents | ID-Token | Attestations | Actions |
|-----|----------|----------|--------------|---------|
| guard | read | - | - | - |
| build | read | write | write | - |
| provenance | write | write | - | read |
| publish | write | - | - | - |

### reusable-rust-release-build.yml

| Job | Contents | ID-Token | Attestations |
|-----|----------|----------|--------------|
| build-release-subjects | read | - | - |
| sbom-and-attest | read | write | write |

### verify-release.yml

| Job | Contents | Attestations |
|-----|----------|--------------|
| verify | read | read |
| verify-slsa | read | - |

## Release Process

### For Maintainers

1. **Prepare Release**
   ```bash
   # Update version in Cargo.toml (workspace root and all crates)
   # Ensure CHANGELOG is updated
   git add -A
   git commit -m "chore: release v0.1.0"
   git push origin main
   ```

2. **Create Tag**
   ```bash
   git tag -s v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

3. **Monitor Release**
   - Watch `release.yml` workflow in GitHub Actions
   - Verify all jobs complete successfully
   - Check that GitHub Release is created with all artifacts

4. **Verify Release** (optional but recommended)
   ```bash
   gh workflow run verify-release.yml -f tag=v0.1.0
   ```

### For Consumers

Download artifacts from the GitHub Release page, then verify:

```bash
# Download artifact
gh release download v0.1.0 -R windlasstech/wibble-client-core

# Verify GitHub attestation
gh attestation verify wibble-client-core-0.1.0.crate \
  -R windlasstech/wibble-client-core

# Verify SBOM attestation
gh attestation verify wibble-client-core-0.1.0.crate \
  -R windlasstech/wibble-client-core \
  --predicate-type https://spdx.dev/Document/v2.3

# Verify SLSA provenance
slsa-verifier verify-artifact wibble-client-core-0.1.0.crate \
  --provenance-path wibble-client-core.intoto.jsonl \
  --source-uri github.com/windlasstech/wibble-client-core \
  --source-tag v0.1.0
```

## Verification Commands

### GitHub Attestation Verification

```bash
# Basic provenance verification
gh attestation verify <ARTIFACT> \
  -R windlasstech/wibble-client-core

# SBOM attestation verification
gh attestation verify <ARTIFACT> \
  -R windlasstech/wibble-client-core \
  --predicate-type https://spdx.dev/Document/v2.3

# Detailed output
gh attestation verify <ARTIFACT> \
  -R windlasstech/wibble-client-core \
  --format json
```

### SLSA Provenance Verification

```bash
# Install slsa-verifier (if not already installed)
curl -sL https://github.com/slsa-framework/slsa-verifier/releases/download/v2.7.0/slsa-verifier-linux-amd64 -o slsa-verifier
chmod +x slsa-verifier

# Verify artifact
./slsa-verifier verify-artifact <ARTIFACT> \
  --provenance-path wibble-client-core.intoto.jsonl \
  --source-uri github.com/windlasstech/wibble-client-core \
  --source-tag v0.1.0
```

### Checksum Verification

```bash
# Verify checksums
sha256sum -c sha256SUMS

# Or manually
cat subject.checksums.txt
sha256sum wibble-client-core-0.1.0.crate
```

## Failure Modes

### Verification Failures

| Symptom | Likely Cause | Resolution |
|---------|--------------|------------|
| `gh attestation verify` fails | Wrong repository or artifact not attested | Ensure artifact is from correct release |
| `slsa-verifier` fails | Wrong source-uri or source-tag | Check tag matches exactly (including `v` prefix) |
| Checksum mismatch | Corrupted download or wrong artifact | Re-download from GitHub Release |
| SBOM attestation not found | SBOM not generated for that release | Check release workflow logs |

### Build Failures

| Symptom | Likely Cause | Resolution |
|---------|--------------|------------|
| Tag/version mismatch | Tag version doesn't match Cargo.toml | Ensure both versions align |
| Guard job fails | Tag not on main branch | Only tags on main are allowed |
| Build job fails | Test failure or compilation error | Fix code and retry |
| Provenance job fails | Generator unable to verify | Check base64-subjects output |

## Security Considerations

### Trust Model

1. **GitHub Actions**: Trusted build service (SLSA L3 compliant)
2. **Reusable Workflow**: Trusted build boundary (this repository)
3. **slsa-github-generator**: Trusted provenance generator (OpenSSF maintained)
4. **Sigstore**: Trusted signing infrastructure (GitHub's private instance for private repos, public-good for public repos)

### Policy Exceptions

- **slsa-github-generator tag reference**: The generator MUST be referenced by semver tag (`@v2.1.0`) instead of SHA. This is required for `slsa-verifier` compatibility and is an approved exception per ADR-0003.

### Audit Trail

All attestations are stored in:
- GitHub Attestations API (viewable via `gh attestation`)
- GitHub Release assets (provenance file)
- Repository Actions logs

## References

- [SLSA v1.2 Build Track Basics](https://slsa.dev/spec/v1.2/build-track-basics)
- [SLSA v1.2 Build Requirements](https://slsa.dev/spec/v1.2/build-requirements)
- [slsa-github-generator](https://github.com/slsa-framework/slsa-github-generator)
- [actions/attest](https://github.com/actions/attest)
- [GitHub Artifact Attestations](https://docs.github.com/en/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds)
- [ADR-0003: SLSA Build L3 Release Pipeline](../decisions/ADR-0003-slsa-level-3-release-pipeline.md)
