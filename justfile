build:
	cargo build

run:
	cargo run -p cli

bench:
	cargo bench --bench insert -p catalog
	cargo bench --bench query -p execution

bench-insert:
	cargo bench --bench insert -p catalog

bench-query:
	cargo bench --bench query -p execution

check:
	cargo clippy --workspace
	cargo build --workspace

test:
	cargo nextest run --workspace --no-tests=pass

fmt:
	cargo fmt
