import struct

# WASM Binary Generator
# Signature: (param i64 i64) (result i32)
# This matches the u64 (converted to i64) that the Orchestrator sends.

wasm_header = b'\x00\x61\x73\x6d\x01\x00\x00\x00'

# Type Section: ID 1
# 1 count
# Type 0: Func
# Params: 2 (i64, i64) -> \x7e \x7e
# Result: 1 (i32) -> \x7f
type_sec = b'\x01\x07\x01\x60\x02\x7e\x7e\x01\x7f'

# Function Section: ID 3
# 1 count, Type Index 0
func_sec = b'\x03\x02\x01\x00'

# Export Section: ID 7
# Export "execute" -> Func Index 0
# Length of "execute" is 7
export_sec = b'\x07\x0b\x01\x07\x65\x78\x65\x63\x75\x74\x65\x00\x00'

# Code Section: ID 10
# 1 count
# Body size: 6 bytes
# Locals: 0
# Opcode: i32.const 42 (0x41 0x2A)
# Opcode: end (0x0b)
code_sec = b'\x0a\x06\x01\x04\x00\x41\x2a\x0b'

with open('valid_sum_i64.wasm', 'wb') as f:
    f.write(wasm_header + type_sec + func_sec + export_sec + code_sec)

print("Generated valid_sum_i64.wasm (Signature: i64, i64 -> i32)")
