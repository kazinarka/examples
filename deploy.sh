cargo build-bpf --manifest-path=./programs/reward_pool/Cargo.toml --bpf-out-dir=./dist/reward_pool
solana program deploy dist/reward_pool/reward_pool.so
cargo build-bpf --manifest-path=./programs/token/Cargo.toml --bpf-out-dir=./dist/token
solana program deploy dist/token/token.so