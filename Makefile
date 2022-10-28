build:
	cd programs/reward_pool; cargo build-bpf

fmt:
	cd programs/reward_pool; cargo  fmt --all

lint:
	cd programs/reward_pool; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: build fmt lint

deploy:
	sh deploy.sh

generate_vault:
	cd client; cargo run -- generate_vault -e dev -s /Users/illiafedotov/.config/solana/id.json

stake:
	cd client; cargo run -- stake -e dev -s /Users/illiafedotov/.config/solana/id.json -m 6DSeL58x2Mu7e9wL2jm8EEarXzWMgj62rmxSen2YAVpY -a 100

unstake:
	cd client; cargo run -- unstake -e dev -s /Users/illiafedotov/.config/solana/id.json -m 6DSeL58x2Mu7e9wL2jm8EEarXzWMgj62rmxSen2YAVpY

stake_nft:
	cd client; cargo run -- stake_nft -e dev -s /Users/illiafedotov/.config/solana/id.json -n [mint_nft_address]

unstake_nft:
	cd client; cargo run -- unstake_nft -e dev -s /Users/illiafedotov/.config/solana/id.json -n [mint_nft_address]