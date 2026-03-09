# Swarm-Runtime: Technical Documentation & Post-Mortems

## The Two Firewalls Trap (Phase 9.2)
When deploying the Gateway to an Oracle Cloud Ubuntu instance, traffic failed to route despite opening ports 3000 and 4000 on the Oracle VCN dashboard.
**The Trap:** Cloud providers implement an external VCN firewall, but Ubuntu OS implements an internal `iptables` firewall pre-configured to block non-SSH traffic.
**Solution:** Explicitly ran `sudo iptables -I INPUT -p tcp --dport 4000 -j ACCEPT` to bypass the internal NAT layer, permitting Libp2p mesh connections.

## The mDNS Local-Only Trap (Phase 9.2)
During the transition to the public cloud, Workers booted without errors but silently failed to dial the Gateway.
**The Trap:** The application relied purely on Multicast DNS (mDNS) for peer discovery. mDNS operates strictly on local area networks (LAN) and cannot cross the public internet.
**Solution:** Injected an explicit `Multiaddr` dial command (`/ip4/<PUBLIC_IP>/tcp/4000/p2p/<GATEWAY_PEER_ID>`) into the Worker's boot sequence, forcing it to route directly to the Cloud over the WAN.

## The PKI Trap: Application vs Network Identity (Phase 9)
**The Trap:** Worker Application keys mismatched Gateway Network keys.
**Solution:** Injected the permanent 32-byte seed directly into the Libp2p `SwarmBuilder`, standardizing identity and relying on Libp2p `Noise` for payload encryption.
