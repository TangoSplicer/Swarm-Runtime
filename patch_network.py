import os

main_path = './components/swarm-node/src/main.rs'
worker_path = './components/swarm-node/src/worker.rs'

# --- 1. Patch main.rs ---
with open(main_path, 'r') as f:
    main_code = f.read()

# Inject the bootnode argument into the CLI struct
main_code = main_code.replace(
    "Start {\n        #[arg(long)]\n        shard: u64,\n    }",
    "Start {\n        #[arg(long)]\n        shard: u64,\n        #[arg(long)]\n        bootnode: String,\n    }"
)

# Extract it in the match arm
main_code = main_code.replace(
    "Commands::Start { shard } => {",
    "Commands::Start { shard, bootnode } => {"
)

# Pass it into the worker thread
main_code = main_code.replace(
    "worker::run_worker(worker_shard, worker_key, worker_seed).await",
    "worker::run_worker(worker_shard, worker_key, worker_seed, bootnode.clone()).await"
)

with open(main_path, 'w') as f:
    f.write(main_code)


# --- 2. Patch worker.rs ---
with open(worker_path, 'r') as f:
    worker_code = f.read()

# Update the function signature to accept the bootnode string
worker_code = worker_code.replace(
    "pub async fn run_worker(shard_id: u64, verifying_key: VerifyingKey, seed: [u8; 32]) -> Result<()> {",
    "pub async fn run_worker(shard_id: u64, verifying_key: VerifyingKey, seed: [u8; 32], bootnode: String) -> Result<()> {"
)

# Inject the dial command right after the node subscribes to the control plane
dial_injection = """p2p_node.subscribe("swarm-control-plane")?;

    if let Ok(addr) = bootnode.parse::<libp2p::Multiaddr>() {
        println!("📞 Dialing Orchestration Gateway: {}", addr);
        let _ = p2p_node.swarm.dial(addr);
    } else {
        eprintln!("⚠️ WARNING: Invalid bootnode multiaddress provided. Node will run in isolation.");
    }
"""
worker_code = worker_code.replace(
    'p2p_node.subscribe("swarm-control-plane")?;',
    dial_injection
)

with open(worker_path, 'w') as f:
    f.write(worker_code)

print("✅ Network connection logic successfully injected!")
