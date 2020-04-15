mod args;
mod term;

use std::io::{
    self,
    Read,
    Write,
};
use args::{Args, Radix};

fn ascii_from_byte(byte: u8) -> Option<char> {
    match byte {
        32..=126 => Some(byte as char),
        _ => None,
    }
}

fn bin_dump(
    mut input : impl Iterator<Item=io::Result<u8>>,
    mut output: impl Write,
    radix     : Radix,
    index     : bool,
    ascii     : bool,
    escape    : bool,
    step      : usize,
    start     : usize,
) -> io::Result<()> {

    let mut buffer = Vec::with_capacity(step);

    for i in (start..).step_by(step) {
        buffer.clear();
        for byte in (&mut input).take(step) {
            buffer.push(byte?);
        }

        if buffer.len() == 0 {
            break
        }

        if index {
            if escape {
                write!(output, "{}{:06x}{}  ", term::BOLD, i, term::RESET_ALL)?;
            }
            else {
                write!(output, "{:06x}  ", i)?;
            }
        }

        let mut have_pred = false;
        for &byte in &buffer {
            if have_pred {
                write!(output, " ")?;
            }
            have_pred = true;
            if escape && byte == 0 {
                write!(output, "{}", term::DIM)?;
            }
            match radix {
                Radix::Bin => {
                    write!(output, "{:08b}", byte)?;
                }
                Radix::Oct => {
                    write!(output, "{:03o}", byte)?;
                }
                Radix::Dec => {
                    write!(output, "{:3}", byte)?;
                }
                Radix::Hex => {
                    write!(output, "{:02x}", byte)?;
                }
            }
            if escape && byte == 0 {
                write!(output, "{}", term::RESET_ALL)?;
            }
        }
        
        if ascii {
            for _ in buffer.len() .. step {
                match radix {
                    Radix::Bin => {
                        write!(output, "         ")?;
                    }
                    Radix::Oct => {
                        write!(output, "    ")?;
                    }
                    Radix::Dec => {
                        write!(output, "    ")?;
                    }
                    Radix::Hex => {
                        write!(output, "   ")?;
                    }
                }
            }
            write!(output, "  ")?;
            let mut text = false;
            if escape {
                write!(output, "{}", term::BOLD)?;
            }
            for &byte in &buffer {
                if escape {
                    match ascii_from_byte(byte) {
                        Some(c) => {
                            if !text {
                                write!(output, "{}{}", term::RESET_ALL, term::DIM)?;
                                text = true;
                            }
                            write!(output, " {}", c)?
                        }
                        None => {
                            if text {
                                write!(output, "{}{}", term::RESET_ALL, term::BOLD)?;
                                text = false;
                            }
                            write!(output, " ·")?
                        }
                    }
                }
                else {
                    match ascii_from_byte(byte) {
                        Some(c) => write!(output, " {}", c)?,
                        None    => write!(output, " .")?,
                    }
                }
            }
            if escape {
                write!(output, "{}", term::RESET_ALL)?;
            }
        }

        writeln!(output)?;
    }

    output.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {

    let Args {
        mut index,
        line: step,
        radix,
        input,
        mut ascii,
        mut escape,
        pretty,
        select: range,
    } = Args::from_args();

    if pretty {
        index  = true;
        ascii  = true;
        escape = true;
    }

    let step = step.get();

    let stdout_guard = std::io::stdout();
    let stdin_guard  = std::io::stdin ();

    let input: Box<dyn std::io::Read> = if let Some(path) = input {
        Box::new(std::fs::File::open(path)?)
    }
    else {
        Box::new(stdin_guard.lock())
    };

    let input = io::BufReader::new(input).bytes();
    let output = io::BufWriter::new(stdout_guard.lock());

    let mut start_at = 0;
    let input: Box<dyn Iterator<Item=io::Result<u8>>> = match range {
        Some(args::IndexRange{start, stop}) => {
            start_at = start;
            let input = input.skip(start);
            if let Some(stop) = stop {
                Box::new(input.take(stop - start))
            }
            else {
                Box::new(input)
            }
        }
        None => Box::new(input),
    };

    match bin_dump(
        input,
        output,
        radix,
        index,
        ascii,
        escape,
        step,
        start_at,
    ) {
        Err(e) if e.kind() != io::ErrorKind::BrokenPipe => Err(e),
        _ => Ok(()),
    }
}
