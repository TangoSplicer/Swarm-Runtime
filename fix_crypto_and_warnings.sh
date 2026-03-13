#!/bin/bash

cd components/swarm-node || exit

# 1. Remove the unneeded rand crate
cargo remove rand

# 2. Fix the unused mutable warning in worker.rs
sed -i 's/mut hash, new_state_opt/hash, new_state_opt/g' src/worker.rs

# 3. Replace rand crate usage with native /dev/urandom in main.rs
sed -i 's/use rand::rngs::OsRng;//g' src/main.rs

sed -i 's/let mut csprng = OsRng;/let mut key_bytes = [0u8; 32];\n        let mut file = std::fs::File::open("\/dev\/urandom").expect("Failed to open \/dev\/urandom");\n        use std::io::Read;\n        file.read_exact(\&mut key_bytes).expect("Failed to read random bytes");/g' src/main.rs

sed -i 's/let key = SigningKey::generate(&mut csprng);/let key = SigningKey::from_bytes(\&key_bytes);/g' src/main.rs

echo "✅ Identity generation restored to /dev/urandom and warnings resolved."
