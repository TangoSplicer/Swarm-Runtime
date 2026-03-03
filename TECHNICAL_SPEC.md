# Swarm Runtime: Technical Specification v0.21.1

## 1. System Architecture
* **Topology:** Physical Mesh (Libp2p), Logical Star (Gateway-Coordinator).
* **Control Plane:** `gossipsub` (Used strictly for `TEL:` hardware heartbeats).
* **Data Plane:** `libp2p::request_response` (1-to-1 Unicast TCP streams) with a strict 2MB Stream Limit.
* **Storage Plane:** `libp2p::kad` (Kademlia DHT for VMFS file pinning).
* **Consensus:** Dynamic Redundancy Factor (Max: 2) + SHA-256 Output State Hashing.

## 2. Dynamic Payload Routing (Interpreted vs Compiled)
The Swarm CLI multiplexes deployments into two major network pipelines to optimize network buffers:

### A. Zero-Extraction Edge Caching (Interpreted)
Used for massive engines (Python, Ruby, PHP).
1. The CLI attaches a `POLYGLOT:LANG` identifier and sends the pure text source.
2. The Worker loads a pre-cached WASI binary (e.g., `php.wasm`) from local storage into memory.
3. The source text is executed against the cached engine inside the `Judge`.

### B. Local compilation & Base64 Transfer (Compiled)
Used for highly-optimized System Languages (Zig, Raw Wasm).
1. The CLI intercepts the command, spawns a child process (`zig build-exe -target wasm32-wasi -O ReleaseSmall`), and generates an ultra-lean `.wasm` file.
2. The CLI encodes the binary as a Base64 string and deploys it with a standard `_start` WASIp1 identifier.
3. The Gateway shards a pseudo-dataset string (`"EXECUTE_NATIVE_WASM"`) to trigger the MapReduce load-balancer.
4. Workers instantly execute the native bytes directly in Wasmi without any Edge Cache lookups.
