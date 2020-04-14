# bin-dump

Inspect a file in binary, octal, decimal or hexadecimal.

```
USAGE:
    bin-dump [FLAGS] [OPTIONS] [input]

FLAGS:
    -a, --ascii      Adds an ASCII interpretation
    -e, --escape     Uses escape sequences to style the output
    -h, --help       Prints help information
    -i, --indices    Adds the bytes index
    -p, --pretty     Enables --indices, --ascii and --escape
    -V, --version    Prints version information

OPTIONS:
    -r, --radix <radix>    Radix to display bytes [default: Bin]  [possible values: Bin, Oct, Dec, Hex]
    -s, --step <step>      How many bytes per line [default: 4]

ARGS:
    <input>    Input file path
```