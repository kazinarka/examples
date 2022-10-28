cargo build-bpf --manifest-path=./programs/reward_pool/Cargo.toml --bpf-out-dir=./dist/reward_pool
solana program deploy dist/reward_pool/reward_pool.so