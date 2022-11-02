build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo  fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

test-say-hello:
	cd program; cargo test-bpf --test say_hello

test: test-say-hello

pre-commit: build fmt lint test

deploy:
	sh deploy.sh

say_hello:
	cd client; cargo run -- say_hello -e dev -s /Users/illiafedotov/.config/solana/id.json
