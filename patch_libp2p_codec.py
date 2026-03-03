import re

path = "components/synapse/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

# We capture the existing Behaviour::new initialization and extract its protocol string
pattern = re.compile(
    r'(request_response::cbor::Behaviour(?:\s*<[^>]+>)?\s*::\s*new\s*\((.*?),\s*request_response::Config::default\(\)\s*\))',
    re.DOTALL
)

def replacer(match):
    protocols = match.group(2)
    # Inject the explicit custom Codec
    return f"""{{
                    let mut codec = request_response::cbor::Codec::default();
                    codec.set_max_request_size(50 * 1024 * 1024);
                    codec.set_max_response_size(50 * 1024 * 1024);
                    request_response::cbor::Behaviour::with_codec(
                        codec,
                        {protocols},
                        request_response::Config::default()
                    )
                }}"""

new_code, count = pattern.subn(replacer, code)

if count > 0:
    with open(path, "w") as f:
        f.write(new_code)
    print("✅ Successfully patched Libp2p CBOR Codec payload limits!")
else:
    print("❌ Could not find the Behaviour::new initialization.")
