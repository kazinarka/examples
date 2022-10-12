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
	cd client; cargo run -- say_hello -e dev -s /home/ideasoft/.config/solana/id.json