import struct

def create_wasm(filename="valid_sum.wasm"):
    # Signature: (param i32 i32) (result i32) -> Returns 42
    wasm_header = b'\x00\x61\x73\x6d\x01\x00\x00\x00'
    type_sec = b'\x01\x07\x01\x60\x02\x7f\x7f\x01\x7f'
    func_sec = b'\x03\x02\x01\x00'
    export_sec = b'\x07\x0b\x01\x07\x65\x78\x65\x63\x75\x74\x65\x00\x00'
    code_sec = b'\x0a\x06\x01\x04\x00\x41\x2a\x0b'

    with open(filename, 'wb') as f:
        f.write(wasm_header + type_sec + func_sec + export_sec + code_sec)
    print(f"Generated {filename}")

if __name__ == "__main__":
    create_wasm()

