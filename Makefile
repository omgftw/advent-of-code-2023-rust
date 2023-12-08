WARMUP=5
BINARY_PATH=./target/release/advent-of-code-2023-rust

build:
	cargo build --release

target/release/advent-of-code-2023-rust:
	cargo build --release

benchmark_day1: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day1'

benchmark_day2: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day2'

benchmark_day3: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day3'

benchmark_day4: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day4'

benchmark_all: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH}'

clean:
	rm -rf target
