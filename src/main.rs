mod args;

use std::io::{
    Read,
    Write,
};
use args::{Args, Radix};

fn main() -> std::io::Result<()> {
    
    let Args {
        indices,
        step,
        radix,
        input,
        ascii: _,
    } = Args::from_args();

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

    let mut buffer = Vec::with_capacity(step);

    for i in (0..).step_by(step) {
        buffer.clear();
        for byte in (&mut input).take(step) {
            buffer.push(byte?);
        }

        if buffer.len() == 0 {
            break
        }

        if indices {
            write!(stdout, "{:04x}  ", i)?;
        }

        for &byte in &buffer {
            // TODO: only match for format string
            match radix {
                Radix::Bin => {
                    write!(stdout, "{:08b} ", byte)?;
                }
                Radix::Oct => {
                    write!(stdout, "{:03o} ", byte)?;
                }
                Radix::Dec => {
                    write!(stdout, "{:03} ", byte)?;
                }
                Radix::Hex => {
                    write!(stdout, "{:02x} ", byte)?;
                }
            }
        }

        writeln!(stdout)?;
    }

    Ok(())
}
