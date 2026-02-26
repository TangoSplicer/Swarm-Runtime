# Swarm Runtime: Project Context & Architecture

## 1. Project Overview
**Name:** Swarm Runtime
**Goal:** A mobile-first, fault-tolerant distributed edge-compute mesh built in Rust.
**Environment:** Actively developed and tested natively on an Android device using Termux. 
**Current Version:** `v0.18.1` (Modular Architecture & Universal Payloads).

## 2. Core Tech Stack
* **Networking:** `rust-libp2p` (mDNS, GossipSub, Request-Response, Identify).
* **API / Web:** `axum` and `tokio`.
* **Execution:** `wasmer` (Singlepass compiler, Gas Metering Middleware).
* **Hashing:** `sha2` (SHA-256 for deterministic consensus).

## 3. Directory Structure
* `swarm-node`: Modular architecture split into `main.rs`, `gateway.rs`, `worker.rs`, and `types.rs`.
* `synapse`: Decoupled P2P Networking Layer.
* `judge`: WebAssembly Execution engine with linear memory byte injection.

## 4. Current Architecture (v0.18.1)
* **Submit:** HTTP POST queues Wasm and `Vec<String>` dataset.
* **Dispatch:** Gateway wraps data in Ed25519 `SignedPayload` and Unicasts to Primary and Secondary peers.
* **Compute:** Worker runs `Singlepass` Wasm, injects String bytes at 1MB offset, hashes output state.
* **Consensus:** Gateway ensures SHA-256 hashes match. Malicious mismatches result in bans.
