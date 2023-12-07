WARMUP=5
BINARY_PATH=./target/release/advent-of-code-2023-rust
target/release/advent-of-code-2023-rust:
	cargo build --release

benchmark_day1: target/release/advent-of-code-2023-rust
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day1'

benchmark_day2: target/release/advent-of-code-2023-rust
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day2'

benchmark_all: target/release/advent-of-code-2023-rust
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH}'

clean:
	rm -rf target

rebuild: clean target/release/advent-of-code-2023-rust
