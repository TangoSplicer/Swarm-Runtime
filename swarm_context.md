# Swarm Runtime: Project Context & Architecture

## 1. Project Overview
**Name:** Swarm Runtime
**Goal:** A mobile-first, fault-tolerant distributed edge-compute mesh built in Rust.
**Environment:** Actively developed and tested natively on an Android device using Termux. 
**Current Version:** `v0.17.1` (Execution Security & Deterministic Consensus completed).

## 2. Core Tech Stack
* **Networking:** `rust-libp2p` (mDNS, GossipSub for telemetry, Request-Response for payload Unicast, Identify).
* **API / Web:** `axum` and `tokio`.
* **Execution:** `wasmer` (Singlepass compiler, Gas Metering Middleware).
* **State Management:** `dashmap` for concurrent hash maps.

## 3. Directory Structure
* `swarm-node`: Gateway/Worker binary, Scheduler, API, Consensus Engine.
* `synapse`: Decoupled P2P Networking Layer.
* `judge`: WebAssembly Execution engine with linear memory and gas traps.

## 4. Current Architecture (v0.17.1)
* **Submit:** HTTP POST queues Wasm and dataset as `unassigned`.
* **Schedule:** Background loop uses `sysinfo` heartbeats to apply Weighted Sharding Math.
* **Dispatch:** Gateway wraps data in Ed25519 `SignedPayload` and Unicasts to Primary and Secondary peers.
* **Compute:** Worker validates signature, runs `Singlepass` metered Wasm, replies via Unicast.
* **Consensus:** Gateway ensures redundant results match before accepting them. Malicious mismatches result in bans.
