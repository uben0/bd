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

// fn _term_have_escape_seq() -> bool {
//     match std::env::var("TERM") {
//         Ok(value) => {
//             eprintln!("TERM={:?}", value);
//             value == "xterm-256color" ||
//             value == "linux"
//         },
//         _ => false,
//     }
// }

// fn pretty_print_byte(output: &mut impl std::io::Write, byte: u8) -> io::Result<()> {
//     let mut byte = byte.reverse_bits();
//     for _ in 0..8 {
//         if byte & 1 == 1 {
//             write!(output, "{}1{}", term::BOLD, term::RESET_ALL)?
//         }
//         else {
//             write!(output, "{}0{}", term::DIM, term::RESET_ALL)?
//         }
//         byte >>= 1;
//     }
//     Ok(())
// }

fn bin_dump(
    input     : impl Read,
    mut output: impl Write,
    radix     : Radix,
    indices   : bool,
    ascii     : bool,
    escape    : bool,
    step      : usize,
) -> io::Result<()> {

    let mut input = input.bytes();
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
            if escape {
                write!(output, "{}{:06x}{}  ", term::BOLD, i, term::RESET_ALL)?;
            }
            else {
                write!(output, "{:06x}  ", i)?;
            }
        }

        for &byte in &buffer {
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
            write!(output, " ")?;
        }

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

        if ascii {
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
                            write!(output, "{} ", c)?
                        }
                        None => {
                            if text {
                                write!(output, "{}{}", term::RESET_ALL, term::BOLD)?;
                                text = false;
                            }
                            write!(output, "· ")?
                        }
                    }
                }
                else {
                    match ascii_from_byte(byte) {
                        Some(c) => write!(output, "{} ", c)?,
                        None    => write!(output, ". ")?,
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
        mut indices,
        step,
        radix,
        input,
        mut ascii,
        mut escape,
        pretty,
    } = Args::from_args();

    if pretty {
        indices = true;
        ascii   = true;
        escape  = true;
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

    match bin_dump(
        io::BufReader::new(input),
        io::BufWriter::new(stdout_guard.lock()),
        radix,
        indices,
        ascii,
        escape,
        step
    ) {
        Err(e) if e.kind() != io::ErrorKind::BrokenPipe => Err(e),
        _ => Ok(()),
    }
}
