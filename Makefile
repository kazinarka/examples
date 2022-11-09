build:
	cd programs/reward_pool; cargo build-bpf

token_build:
	cd programs/token; cargo build-bpf

fmt:
	cargo fmt --all

lint:
	cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: build fmt lint

deploy:
	sh deploy.sh

generate_vault:
	cd client; cargo run -- generate_vault -e dev -s [path to wallet]

stake:
	cd client; cargo run -- stake -e dev -s [path to wallet] -m [token mint] -a [value]

unstake:
	cd client; cargo run -- unstake -e dev -s [path to wallet] -m [token mint]

stake_nft:
	cd client; cargo run -- stake_nft -e dev -s [path to wallet] -n [nft mint]

unstake_nft:
	cd client; cargo run -- unstake_nft -e dev -s [path to wallet] -n [nft mint]

generate_token:
	cd client; cargo run -- generate_token -e dev -s [path to wallet]
