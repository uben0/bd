use structopt::StructOpt;

#[derive(Debug)]
pub enum Radix {
    Bin,
    Oct,
    Dec,
    Hex,
}

impl std::str::FromStr for Radix {
    type Err = String;
    fn from_str(src: &str) -> Result<Self, String> {
        match src {
            "2"  | "bin" | "Bin" | "BIN" => Ok(Radix::Bin),
            "8"  | "oct" | "Oct" | "OCT" => Ok(Radix::Oct),
            "10" | "dec" | "Dec" | "DEC" => Ok(Radix::Dec),
            "16" | "hex" | "Hex" | "HEX" => Ok(Radix::Hex),
            _ => Err("unknown radix".to_string())
        }
    }
}

#[derive(Debug)]
pub struct IndexRange {
    pub start: usize,
    pub stop: Option<usize>,
}

#[derive(Debug)]
pub struct ParseIndexRangeError;

impl std::fmt::Display for ParseIndexRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        "expects 2 unsigned integers seperated by \"..\"".fmt(f)
    }
}

impl std::str::FromStr for IndexRange {
    type Err = ParseIndexRangeError;
    fn from_str(src: &str) -> Result<Self, ParseIndexRangeError> {
        let mut iter = src.split("..");
        match [iter.next(), iter.next(), iter.next()] {
            [Some(start), Some(stop), None] => {
                let (start, stop) = (start.trim(), stop.trim());
                let start = if start == "" {0   } else {     start.parse().map_err(|_| ParseIndexRangeError)? };
                let stop  = if stop  == "" {None} else {Some(stop .parse().map_err(|_| ParseIndexRangeError)?)};
                if let Some(stop) = stop {
                    if stop < start {
                        return Err(ParseIndexRangeError)
                    }
                }
                Ok(IndexRange{start, stop})
            }
            _ => Err(ParseIndexRangeError),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Inspect a file in binary, octal, decimal or hexadecimal.")]
pub struct Args {
    /// Specify a input file path, if none, standard input is used
    #[structopt(name = "input_file")]
    pub input: Option<std::path::PathBuf>,

    /// Adds the bytes index
    #[structopt(short, long)]
    pub index: bool,

    /// How many bytes per line
    #[structopt(short, long, name = "bytes", default_value="8")]
    pub line: std::num::NonZeroUsize,

    /// Radix to display bytes [possible values: bin, oct, dec, hex]
    #[structopt(
        short,
        long,
        name = "radix",
        default_value="bin",
    )]
    pub radix: Radix,

    /// Adds an ASCII interpretation
    #[structopt(short, long)]
    pub ascii: bool,

    /// Uses escape sequences to style the output
    #[structopt(short, long)]
    pub escape: bool,

    /// Enables --index, --ascii and --escape
    #[structopt(short, long)]
    pub pretty: bool,

    /// Only prints a given range of bytes [examples: 64..92, ..1024, 1234..]
    #[structopt(short, long, name="range")]
    pub select: Option<IndexRange>,
}

impl Args {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}