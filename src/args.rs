use structopt::StructOpt;
use clap::arg_enum;


arg_enum! {
    #[derive(Debug)]
    pub enum Radix {
        Bin,
        Oct,
        Dec,
        Hex,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Inspect a file in binary, octal, decimal or hexadecimal.")]
pub struct Args {
    /// Input file path
    pub input: Option<std::path::PathBuf>,

    /// Prints memory address
    #[structopt(short, long)]
    pub indices: bool,

    /// How many bytes per line
    #[structopt(short, long, default_value="4")]
    pub step: std::num::NonZeroUsize,

    /// Radix to display bytes
    #[structopt(short, long, possible_values = &Radix::variants(), case_insensitive = true, default_value="Bin")]
    pub radix: Radix,

    /// Prints ASCII interpretation
    #[structopt(short, long)]
    pub ascii: bool,
}

impl Args {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}