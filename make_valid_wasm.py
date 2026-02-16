import struct

# This is a minimal WASM binary that exports a function "execute"
# Signature: (param i32 i32) (result i32)
# Logic: Returns 42 (Hardware encoded)
wasm_header = b'\x00\x61\x73\x6d\x01\x00\x00\x00'
type_sec = b'\x01\x07\x01\x60\x02\x7f\x7f\x01\x7f'  # Type 0: (i32, i32) -> i32
func_sec = b'\x03\x02\x01\x00'                     # Func 0 is Type 0
export_sec = b'\x07\x0b\x01\x07\x65\x78\x65\x63\x75\x74\x65\x00\x00' # Export "execute" -> Func 0
code_sec = b'\x0a\x06\x01\x04\x00\x41\x2a\x0b'     # Func 0 body: i32.const 42, end

with open('valid_sum.wasm', 'wb') as f:
    f.write(wasm_header + type_sec + func_sec + export_sec + code_sec)
print("Generated valid_sum.wasm (Returns 42)")
