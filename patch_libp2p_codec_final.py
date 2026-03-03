path = "components/synapse/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

target = """                let req_res_protocol = StreamProtocol::new("/swarm/req-res/1.0.0");
                let req_res_config = request_response::Config::default().with_request_timeout(std::time::Duration::from_secs(300));
                let req_res = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::new(
                    [(req_res_protocol, request_response::ProtocolSupport::Full)],
                    req_res_config,
                );"""

replacement = """                let req_res_protocol = StreamProtocol::new("/swarm/req-res/1.0.0");
                let req_res_config = request_response::Config::default().with_request_timeout(std::time::Duration::from_secs(300));
                
                let mut codec = request_response::cbor::Codec::default();
                codec.set_max_request_size(50 * 1024 * 1024);
                codec.set_max_response_size(50 * 1024 * 1024);
                
                let req_res = request_response::cbor::Behaviour::<SwarmRequest, SwarmResponse>::with_codec(
                    codec,
                    [(req_res_protocol, request_response::ProtocolSupport::Full)],
                    req_res_config,
                );"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Successfully patched Libp2p CBOR Codec payload limits!")
else:
    print("❌ Could not find the target block. Double-check indentation.")
