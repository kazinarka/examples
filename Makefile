build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo  fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: build fmt lint

deploy:
	sh deploy.sh

say_hello:
	cd client; cargo run -- say_hello -e dev -s /path/to/id.json

generate_vault:
	cd client; cargo run -- generate_vault -e dev -s /path/to/id.json

stake:
	cd client; cargo run -- stake -e dev -s /path/to/id.json -m [token_mint_address] -a [amount]

unstake:
	cd client; cargo run -- unstake -e dev -s /path/to/id.json -m [token_mint_address]

stake_nft:
	cd client; cargo run -- stake_nft -e dev -s /path/to/id.json -n [mint_nft_address]

unstake_nft:
	cd client; cargo run -- unstake_nft -e dev -s /path/to/id.json -n [mint_nft_address]
