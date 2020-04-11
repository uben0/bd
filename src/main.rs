use std::io::{
    Read,
    Write,
};
use structopt::StructOpt;
use clap::arg_enum;
arg_enum! {
    #[derive(Debug)]
    enum Radix {
        Bin,
        Oct,
        Dec,
        Hex,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Inspect a file in binary, octal, decimal or hexadecimal.")]
struct Opt {
    /// Input file path
    input: Option<std::path::PathBuf>,

    /// Display memory address
    #[structopt(short, long)]
    indices: bool,

    /// How many bytes per line
    #[structopt(short, long, default_value="4")]
    step: std::num::NonZeroUsize,

    /// Radix to display bytes
    #[structopt(short, long, possible_values = &Radix::variants(), case_insensitive = true, default_value="Bin")]
    radix: Radix,
}

fn main() -> std::io::Result<()> {
    
    
    let Opt {
        indices,
        step,
        radix,
        input,
    } = Opt::from_args();

    let step = step.get();

    let stdout_guard = std::io::stdout();
    let stdin_guard  = std::io::stdin ();
    
    let input: Box<dyn std::io::Read> = if let Some(path) = input {
        Box::new(std::fs::File::open(path)?)
    }
    else {
        Box::new(stdin_guard.lock())
    };

    let mut input = std::io::BufReader::new(input).bytes();

    let mut stdout = stdout_guard.lock();

    for i in (0..).step_by(step) {
        let mut count = 0;
        for byte in (&mut input).take(step) {
            if indices && count == 0 {
                write!(stdout, "{:04x}  ", i)?;
            }
            match radix {
                Radix::Bin => {
                    write!(stdout, "{:08b} ", byte?)?;
                }
                Radix::Oct => {
                    write!(stdout, "{:03o} ", byte?)?;
                }
                Radix::Dec => {
                    write!(stdout, "{:03} ", byte?)?;
                }
                Radix::Hex => {
                    write!(stdout, "{:02x} ", byte?)?;
                }
            }
            count += 1;
        }
        if count != 0 {
            writeln!(stdout)?;
        }
        if count != step {
            break;
        }
    }

    Ok(())
}
