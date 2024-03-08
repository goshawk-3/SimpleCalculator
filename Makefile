release: ## Build library
	cargo build --release

release_cli: ## Build binary
	cargo build --release --bin calc_cli

run_cli: ## Build library
	cargo run --release --bin calc_cli


test: ## Test library
	cargo test -p calculator


