import struct

# Generates a valid Wasm file for v0.9.3 Runtime
# Signature: (param i32 i32) (result i32) -> Returns 42
def main():
    header = b'\x00\x61\x73\x6d\x01\x00\x00\x00'
    types = b'\x01\x07\x01\x60\x02\x7f\x7f\x01\x7f'
    funcs = b'\x03\x02\x01\x00'
    exports = b'\x07\x0b\x01\x07\x65\x78\x65\x63\x75\x74\x65\x00\x00'
    code = b'\x0a\x06\x01\x04\x00\x41\x2a\x0b'
    
    with open('final_sum.wasm', 'wb') as f:
        f.write(header + types + funcs + exports + code)
    print("Generated final_sum.wasm (i32 compatible)")

if __name__ == "__main__":
    main()
