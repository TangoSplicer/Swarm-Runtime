# Swarm Runtime security operations

This guide describes the security and operational contract implemented on the current `main` branch. Read it before operating a gateway, deploying workers, or granting a client permission to submit jobs.

## Admission authority

A gateway accepts a multipart job request only when it includes a versioned **Ed25519 admission token**. The token covers the exact payload, serialized metadata, nonce, and expiry. Tokens are valid for at most five minutes, and the gateway records each accepted nonce to reject replays.

Create an admission private key with the unified CLI:

```bash
cargo run --bin swarm-node -- generate-admission-key --output .swarm_admission.key
```

On Unix, this command creates a 32-byte key with owner-only (`0600`) permissions and prints the matching hexadecimal public key. Store the private key in a secret manager or an owner-readable file. Give the gateway **only** the emitted public key:

```bash
cargo run --bin swarm-node -- gateway \
  --port 3000 \
  --admission-public-key <64-character-hex-public-key>
```

Deploy through the unified client with the private admission key:

```bash
cargo run --bin swarm-node -- deploy program.py \
  --lang python \
  --admission-private-key .swarm_admission.key \
  --gateways http://127.0.0.1:3000
```

The standalone CLI uses the same signing contract:

```bash
cargo run --bin swarm-cli -- deploy program.py \
  --lang python \
  --admission-private-key .swarm_admission.key \
  --gateway http://127.0.0.1:3000
```

> Do not commit, bake into images, or distribute the admission private key to worker nodes. It is a control-plane credential, not a node identity.

## Worker and result trust

Workers require `--trusted-gateway`, a hexadecimal Ed25519 public key, before accepting dispatch. Each worker now signs a canonical `SignedShardResult` envelope with its persistent node identity. The gateway verifies the signature before checking assignment, de-duplicating a peer, or considering the result for consensus.

## Runtime selection and local catalog

Runtime selection is explicit in signed deployment metadata. The valid values are `wasm`, `python`, `javascript`, `lua`, `ruby`, `php`, and `sqlite`.

| Runtime kind | Job payload | Local worker requirement |
|---|---|---|
| `wasm` | A valid Wasm module | No catalog artifact; the submitted module executes directly. |
| `python` | Empty payload; source in metadata | `./runtimes/python.wasm` |
| `javascript` | Empty payload; source in metadata | `./runtimes/qjs.wasm` |
| `lua` | Empty payload; source in metadata | `./runtimes/lua.wasm` |
| `ruby` | Empty payload; source in metadata | `./runtimes/ruby.wasm` |
| `php` | Empty payload; source in metadata | `./runtimes/php.wasm` |
| `sqlite` | Empty payload; source in metadata | `./runtimes/sqlite.wasm` |

Install the same reviewed runtime artifacts on all workers and control them outside the job channel. A future release should pin a signed runtime manifest containing artifact version and SHA-256 digest.

## Execution boundary and limits

A worker creates a new shard workspace beneath `./swarm-workspaces/<job-id>/<shard-index>/<random-id>`. The WASI guest receives only this directory as a preopen and accesses state through `/data/state.json`. The worker removes the workspace after constructing the result.

The current Judge configuration limits the Wasm module to 25 MiB and dataset, state, and output artifacts to 5 MiB each. It uses a bounded fuel budget of 50,000,000 units. These are important application controls but are not a substitute for host controls. Run workers in a container or process supervisor with memory, wall-clock, PID, filesystem, and egress-network restrictions.

## Dependency posture

The gateway configures Axum 0.8 with HTTP/1 only and disables broad Libp2p defaults, enabling only the explicit transports and behaviours used by the runtime. In particular, the Libp2p feature lists must not add `mdns`, `tls`, `quic`, or `websocket`; the deployed transport uses TCP, Noise, and Yamux. Axum exposes `http1` and `http2` as distinct optional features.[1]

Run these checks before accepting dependency changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo audit --file Cargo.lock --ignore RUSTSEC-2026-0118 --ignore RUSTSEC-2026-0119
```

The Judge executor uses the stable Wasmi/WASI 1.1 runtime line and capability filesystem stack 3.4.5. The current Axum 0.8 and Libp2p 0.56 migration removed the prior H2 0.3, Ring 0.16, and Rustls-WebPKI 0.101 advisory branches from the lockfile.

### Approved temporary RustSec exceptions

The security workflow contains exactly two explicitly named RustSec exceptions, both retained by the **inactive** optional `libp2p-mdns 0.48.0 → hickory-proto 0.25.2` branch. Neither `swarm-node` nor `synapse` enables the `mdns` feature, and reverse dependency inspection of the active build has no path to either package.

| Advisory | Reason and compensating control | Review deadline |
|---|---|---|
| `RUSTSEC-2026-0118` | Hickory reports no patched 0.25 release. The vulnerable `DnssecDnsHandle` path additionally requires a DNSSEC feature and DNSSEC validation; this project enables neither mDNS nor Hickory DNSSEC. Keep `mdns` absent. | **20 November 2026** |
| `RUSTSEC-2026-0119` | The fix is Hickory 0.26.1, while the latest Libp2p 0.56 optional mDNS component resolves Hickory 0.25.2. The branch is inactive because `mdns` remains disabled. | **20 November 2026** |

These two identifiers are an exception list, not a general audit suppression policy. Remove both ignores immediately when a compatible Libp2p release no longer retains `hickory-proto 0.25.x`; if `mdns` is ever enabled, remove the exception first and upgrade the stack before deployment.[2] [3]

## Remaining high-priority work

The result signature key is verified but is not yet cryptographically registered to the Libp2p `PeerId`. Add a worker-registration attestation binding these keys before treating a result signer as a stable worker identity. In addition, pin runtime artifact hashes and add an end-to-end test covering valid admission, replay rejection, invalid worker signatures, state handling, and workspace cleanup.

## References

[1]: https://docs.rs/axum/0.8.9/axum/#feature-flags "Axum 0.8.9 feature flags"
[2]: https://rustsec.org/advisories/RUSTSEC-2026-0118 "RUSTSEC-2026-0118"
[3]: https://rustsec.org/advisories/RUSTSEC-2026-0119 "RUSTSEC-2026-0119"
