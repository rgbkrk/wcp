# `wcp`: Word Count Performance

`wcp` is a high-performance, Rust-based alternative to the classic Unix `wc` (word count) command. This was made for fun based on a [DuckDB tweet about how much faster than `wc` they are at  counting lines in a CSV](https://x.com/duckdb/status/1863612554896941404).

## Features

- Lightning-fast performance, outperforming both standard `wc` and DuckDB's CSV line counting

- Utilizes advanced optimization techniques:
  - Memory mapping
  - Parallel processing
  - SIMD (Single Instruction, Multiple Data) operations
  - Rust nightly features for additional optimizations

## Performance

In our tests with a 3 GB CSV file ([2023 Railway Services dataset from the Netherlands](https://duckdb.org/2024/05/31/analyzing-railway-traffic-in-the-netherlands.html)):

- `wcp`: 0.112 seconds (1298% CPU usage)
- `wc`: 2.966 seconds (99% CPU usage)
- DuckDB: 1.261 seconds (930% CPU usage)

`wcp` is approximately 26.5 times faster than `wc` and 11.3 times faster than DuckDB for this specific task.

## Usage

```
wcp [OPTIONS] <FILE>

Options:
  -l, --lines    Count lines
  -w, --words    Count words
  -c, --bytes    Count bytes
  -h, --help     Print help
  -V, --version  Print version
```

## Building

Ensure you have Rust nightly installed, then:

```
cargo build --release
```

## How It Works

`wcp` achieves its impressive speed through:

1. Memory mapping the input file for fast access
2. Utilizing parallel processing to handle large chunks of data simultaneously
3. Employing SIMD operations for efficient character counting
4. Optimizing the build with Link Time Optimization (LTO) and single codegen unit

## Compatibility with `wc`

While `wcp` aims to be a high-performance alternative to `wc`, there are some differences in functionality:

1. No stdin support: Currently, `wcp` does not support reading from standard input. It only works with file inputs.

2. Limited options: `wcp` implements the core functionality of `wc` (-l, -w, -c options), but may not support all the extended options that some `wc` implementations provide.

3. Multibyte character handling: The current implementation may not correctly handle multibyte characters or different encodings. It's optimized for ASCII and UTF-8 encoded files.

4. Default behavior: Unlike `wc`, which prints line, word, and byte counts by default if no option is specified, `wcp` requires explicit options.
