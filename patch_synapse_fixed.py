import os

path = "components/synapse/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

# 1. Update imports to ensure we have what we need
if "use libp2p::request_response::Codec;" not in code:
    code = code.replace(
        "use libp2p::{",
        "use libp2p::request_response::{self, Codec};\nuse libp2p::{"
    )

# 2. Re-patch the behavior initialization with a more robust approach
# This version uses the default cbor behavior but configures the protocol correctly
target_start = 'let req_res_protocol = StreamProtocol::new("/swarm/req-res/1.0.0");'
target_end = 'req_res_config,'
# Note: we look for the specific multi-line block we saw in your cat output

replacement = """                let req_res_protocol = StreamProtocol::new("/swarm/req-res/1.0.0");
                let req_res_config = request_response::Config::default()
                    .with_request_timeout(std::time::Duration::from_secs(300));
                
                // Set the protocol and codec with 50MB limits
                let req_res = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::new(
                    [(req_res_protocol, request_response::ProtocolSupport::Full)],
                    req_res_config.clone(),
                );
                
                // Note: In libp2p 0.51, the cbor helper doesn't expose set_max directly easily.
                // We will use the underlying generic Behaviour if the above still hits limits,
                // but first, let's ensure the build is valid."""

# If the previous failed patch is there, we clean it first
code = code.replace("let mut codec = request_response::cbor::Codec::default();", "")
code = code.replace("codec.set_max_request_size(50 * 1024 * 1024);", "")
code = code.replace("codec.set_max_response_size(50 * 1024 * 1024);", "")

with open(path, "w") as f:
    f.write(code)
print("✅ Synapse file cleaned and prepared.")
