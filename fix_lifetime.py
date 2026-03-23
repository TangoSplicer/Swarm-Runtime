import os

main_path = './components/swarm-node/src/main.rs'

with open(main_path, 'r') as f:
    code = f.read()

# 1. Clone the string OUTSIDE the async block to satisfy the borrow checker
code = code.replace(
    "tokio::spawn(async move {",
    "let worker_bootnode = bootnode.clone();\n            tokio::spawn(async move {"
)

# 2. Pass the owned clone INTO the async block
code = code.replace(
    "worker::run_worker(worker_shard, worker_key, worker_seed, bootnode.clone()).await",
    "worker::run_worker(worker_shard, worker_key, worker_seed, worker_bootnode).await"
)

with open(main_path, 'w') as f:
    f.write(code)

print("✅ Lifetime borrow checker conflict resolved!")
