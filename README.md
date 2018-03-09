# wrdcntr - a FAST word counter
This is a simple yet very fast word counter written in [Rust][rust].

The counter reads all text from a given file, counts each word,
and prints all words in alphabetical order with the number of times the word
occurred in the file.

All counting work is split across all CPU cores to count as quickly as possible,
and is done in a [map/reduce][mapreduce]-like manner.

Words contain any alpha numeric character, and are separated by any
non-alphanumeric character. Spaces, punctuation and so on are not counted.

The goal of this project is to show how powerful the Rust language can be with
minimal effort.

## Usage
To use the word counter, supply a file:
```bash
# Count words
wrdcntr some_text.txt
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


[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[mapreduce]: https://en.wikipedia.org/wiki/MapReduce
