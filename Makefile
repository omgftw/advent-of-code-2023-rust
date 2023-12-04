WARMUP=5
BINARY_PATH=./target/release/advent-of-code-2023-rust
target/release/advent-of-code-2023-rust:
	cargo build --release

benchmark_day1: target/release/advent-of-code-2023-rust
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --day1'