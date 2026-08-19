# Releasing Swarm Runtime

The `Release and publish` workflow runs when a tag matching `v*` is pushed, or when it is manually dispatched for an existing tag. It verifies the tag version, runs formatting, strict Clippy, and locked workspace tests, packages the publishable crates, publishes them in dependency order, builds release binaries, and creates a GitHub release.

## One-time repository setup

Create a GitHub environment named **`crates-io-publish`** and configure the approval policy appropriate for the project. Store a crates.io API token with publish permission as the environment secret **`CARGO_REGISTRY_TOKEN`**. The workflow deliberately does not use a token from source control or a tag body.

The coverage workflow uploads LCOV to Codecov. For a private repository or a Codecov organization that requires authentication, add **`CODECOV_TOKEN`** as a repository or environment secret. The coverage artifact remains available in GitHub Actions even when the Codecov upload is unavailable.

## Package metadata prerequisite

The current workspace does not declare a software license file or package `license` metadata. Choose and add the project’s intended license before enabling a production crates.io publication. Do not use a release tag to make this decision implicitly.

The release workflow currently targets these crates in dependency order:

| Order | Crate | Role |
|---:|---|---|
| 1 | `synapse` | Libp2p transport and admission-token protocol library |
| 2 | `judge` | Isolated WASI execution library |
| 3 | `lazarus` | Runtime monitoring library |
| 4 | `swarm-cli` | Deployment and status command-line client |
| 5 | `swarm-node` | Gateway and worker command-line runtime |

`stateful-counter` is marked `publish = false`, because it is a test payload rather than a released crate.

## Creating a release

Update all intended package versions and ensure the tag version matches the `synapse` package version. For example, a `1.0.1` release uses the tag `v1.0.1`.

```bash
git tag -a v1.0.1 -m "Swarm Runtime v1.0.1"
git push origin v1.0.1
```

The environment protection rule, if configured, pauses the publication job for approval. Publishing is intentionally ordered so `synapse`, `judge`, and `lazarus` are available before the CLI and node packages, which use them as dependencies.

> Treat a published version as immutable. Correct a release through a new version and tag rather than deleting or reusing a crates.io version.
