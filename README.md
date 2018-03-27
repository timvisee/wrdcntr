# wrdcntr - a fast word counter in Rust
This is a simple yet very fast word counter written in [Rust][rust].

The counter reads all text from a given file, counts all words,
and prints the number of occurrences for each word in alphabetical order.

All counting work is split across all CPU cores to count as quickly as possible,
and is done in a [map/reduce][mapreduce]-like manner.

Words contain any alpha numeric character, and are separated by any
non-alphanumeric character. Spaces, punctuation and so on are not counted.

The goal of this project is to show how powerful the Rust language can be with
minimal effort.

## Benchmark
Here are some basic benchmarks with files from the [`samples`](samples/)
directory.  
These benchmarks were done with the [hyperfine][hyperfine] tool,
each timing 10 runs.

```
# 511KB with 115478 words
hyperfine 'wrdcntr samples/book.txt --no-output'
# Time (mean ± σ):   10.4 ms ±  1.4 ms
# Range (min … max):  8.7 ms … 19.0 ms
# [User: 22.0 ms, System: 2.4 ms]

# 30MB with 7205401 words
hyperfine 'wrdcntr samples/many_books.txt --no-output'
# Time (mean ± σ):   344.6 ms ±   9.9 ms
# Range (min … max): 333.9 ms … 370.2 ms
# [User: 865.0 ms, System: 37.7 ms]

# 35KB with 7074 words
hyperfine 'wrdcntr LICENSE --no-output'
# Time (mean ± σ):   2.4 ms ±  1.0 ms    
# Range (min … max): 1.3 ms … 12.5 ms
# [User: 3.2 ms, System: 1.2 ms]
# Note: possibly inaccurate, because timings were less than 5ms
```

These benchmarks were done on a machine running Linux with a
4-core i5-4670K @4.1Ghz CPU and 16GB RAM.

Counting files of 1GB is also fast, and nicely saturates all cores:

![Counting 1GB of words on a 32-core server](cpu-usage.png)

## Usage
To use the word counter, supply a file:
```bash
# Count words
wrdcntr samples/book.txt
```

Which outputs:

```
...
About: 12
Above: 1
Absolutely: 1
Actually: 1
Add: 1
Admiration: 1
Adrenaline: 1
After: 37
Again: 1
...
```

## Installation
For installation, the project must be compiled from source.
Git and Rust cargo are required.
Install the latest version of Rust with [rustup][rustup].

Then, clone and install `wrdcntr` with:

```bash
# Clone the project
git clone https://github.com/timvisee/wrdcntr.git
cd wrdcntr

# Install wrdcntr
cargo install -f

# Start using wrdcntr
wrdcntr --help

# or run it directly from Cargo
cargo run --release -- --help
```

Or just build and invoke the binary directly (Linux/macOS):

```bash
# Clone the project
git clone https://github.com/timvisee/wrdcntr.git
cd wrdcntr

# Build the project (release version)
cargo build --release

# Start using wrdcntr
./target/release/wrdcntr --help
```

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.


[hyperfine]: https://github.com/sharkdp/hyperfine
[mapreduce]: https://en.wikipedia.org/wiki/MapReduce
[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
