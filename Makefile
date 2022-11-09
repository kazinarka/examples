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
	cd client; cargo run -- generate_vault -e dev -s /Users/glebprotasov/.config/solana/id.json

stake:
	cd client; cargo run -- stake -e dev -s /Users/glebprotasov/.config/solana/id.json -m 3aH3TVcDvTNRr8c9gX2o7qxjZ4ktufwRLVhDhiZtPAWU -a 1

unstake:
	cd client; cargo run -- unstake -e dev -s /Users/glebprotasov/.config/solana/id.json -m 3aH3TVcDvTNRr8c9gX2o7qxjZ4ktufwRLVhDhiZtPAWU

stake_nft:
	cd client; cargo run -- stake_nft -e dev -s /Users/glebprotasov/.config/solana/id.json -n DtrtDnLdYnmiPbZe8hqFXM7k61cTeQQr8qxPraUBwU8s

unstake_nft:
	cd client; cargo run -- unstake_nft -e dev -s /Users/glebprotasov/.config/solana/id.json -n DtrtDnLdYnmiPbZe8hqFXM7k61cTeQQr8qxPraUBwU8s

generate_token:
	cd client; cargo run -- generate_token -e dev -s /Users/glebprotasov/.config/solana/id.json