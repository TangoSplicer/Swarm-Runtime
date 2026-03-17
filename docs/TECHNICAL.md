# Swarm-Runtime: Technical Documentation & Post-Mortems

## The ARG_MAX Terminal Trap (Phase 10)
**The Trap:** Deploying smart contracts by converting `.wasm` binaries into Base64 strings and stuffing them into JSON payloads caused the Android Termux shell to crash with an `ARG_MAX` buffer overflow.
**Solution:** Eradicated Base64 encoding. Transitioned the Axum Gateway and the `reqwest` CLI to stream raw binary `Vec<u8>` payloads asynchronously using `multipart/form-data`.

## The Unbounded Memory Leak (Phase 10)
**The Trap:** Using `tokio::sync::mpsc::unbounded_channel` meant that if a malicious actor flooded the mesh, the Android edge devices would queue tasks indefinitely until they ran out of RAM and crashed.
**Solution:** Implemented Asynchronous Backpressure. Transitioned to `mpsc::channel(1000)` and swapped `.send()` for `.try_send()`. If the queue hits 1,000 tasks, the node automatically load-sheds and drops non-critical packets.

## The Two Firewalls Trap (Phase 9.2)
When deploying the Gateway to an Oracle Cloud Ubuntu instance, traffic failed to route despite opening ports 3000 and 4000 on the Oracle VCN dashboard.
**The Trap:** Cloud providers implement an external VCN firewall, but Ubuntu OS implements an internal `iptables` firewall pre-configured to block non-SSH traffic.
**Solution:** Explicitly ran `sudo iptables -I INPUT -p tcp --dport 4000 -j ACCEPT`.

## The mDNS Local-Only Trap (Phase 9.2)
**The Trap:** The application relied purely on Multicast DNS (mDNS) for peer discovery, which cannot cross the public internet.
**Solution:** Injected an explicit `Multiaddr` dial command forcing it to route directly to the Cloud over the WAN.

## The Tokio Mutability Trap (Phase 12)
**The Trap:** Attempting to lock the global `DashMap` ledger inside the Libp2p `Swarm` event loop or across an `await` point causes Tokio thread starvation and complete server deadlock.
**Solution:** The "Clone and Release" mandate. The Gateway clones the `Arc<DashMap>`, drops the lock instantly, and offloads JSON replication parsing to a separate `tokio::spawn` task.

## The Permissive Gossip & Infinite Timeout Trap (Phase 12)
**The Trap:** `request_response` channels defaulting to 300s timeouts caused massive memory leaks when Termux workers dropped cellular connections. Furthermore, permissive Gossipsub allowed potential BFT poisoning.
**Solution:** Reduced timeouts strictly to 15s. Locked Gossipsub `ValidationMode` to `Strict` to cryptographically verify every packet in the control plane.
