# bin-dump

Inspect a file in binary, octal, decimal or hexadecimal.

```
USAGE:
    bin-dump [FLAGS] [OPTIONS] [input_file]

FLAGS:
    -a, --ascii      Adds an ASCII interpretation
    -e, --escape     Uses escape sequences to style the output
    -h, --help       Prints help information
    -i, --index      Adds the bytes index
    -p, --pretty     Enables --index, --ascii and --escape
    -V, --version    Prints version information

OPTIONS:
    -l, --line <bytes>      How many bytes per line [default: 8]
    -r, --radix <radix>     Radix to display bytes [possible values: bin, oct, dec, hex] [default: bin]
    -s, --select <range>    Only prints a given range of bytes [examples: 64..92, ..1024, 1234..]

ARGS:
    <input_file>    Specify a input file path, if none, standard input is used
```

## Install

1. Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Compile using `cargo build --release`.
3. Copy `target/release/bin-dump` in your `bin` directory.