# bin-dump

Inspect a file in binary, octal, decimal or hexadecimal.

```
USAGE:
    bin-dump [FLAGS] [OPTIONS] [input]

FLAGS:
    -h, --help       Prints help information
    -i, --indices    Display memory address
    -V, --version    Prints version information

OPTIONS:
    -r, --radix <radix>    Radix to display bytes [default: Bin]  [possible values: Bin, Oct, Dec, Hex]
    -s, --step <step>      How many bytes per line [default: 4]

ARGS:
    <input>    Input file path
```