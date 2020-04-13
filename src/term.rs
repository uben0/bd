#![allow(dead_code)]

pub const RESET_ALL       : &str = "\x1b[0m"  ;
pub const BOLD            : &str = "\x1b[1m"  ;
pub const DIM             : &str = "\x1b[2m"  ;
pub const UNDERLINED      : &str = "\x1b[4m"  ;
pub const BLINK           : &str = "\x1b[5m"  ;
pub const INVERTED        : &str = "\x1b[7m"  ;
pub const HIDDEN          : &str = "\x1b[8m"  ;

pub const RESET_BOLD      : &str = "\x1b[21m" ;
pub const RESET_DIM       : &str = "\x1b[22m" ;
pub const RESET_UNDERLINED: &str = "\x1b[24m" ;
pub const RESET_BLINK     : &str = "\x1b[25m" ;
pub const RESET_INVERTED  : &str = "\x1b[27m" ;
pub const RESET_HIDDEN    : &str = "\x1b[28m" ;

pub const FG_BLACK        : &str = "\x1b[30m" ;
pub const FG_RED          : &str = "\x1b[31m" ;
pub const FG_GREEN        : &str = "\x1b[32m" ;
pub const FG_YELLOW       : &str = "\x1b[33m" ;
pub const FG_BLUE         : &str = "\x1b[34m" ;
pub const FG_MAGENTA      : &str = "\x1b[35m" ;
pub const FG_CYAN         : &str = "\x1b[36m" ;
pub const FG_LIGHT_GRAY   : &str = "\x1b[37m" ;
pub const FG_DEFAULT      : &str = "\x1b[39m" ;

pub const FG_DARK_GRAY    : &str = "\x1b[90m" ;
pub const FG_LIGHT_RED    : &str = "\x1b[91m" ;
pub const FG_LIGHT_GREEN  : &str = "\x1b[92m" ;
pub const FG_LIGHT_YELLOW : &str = "\x1b[93m" ;
pub const FG_LIGHT_BLUE   : &str = "\x1b[94m" ;
pub const FG_LIGHT_MAGENTA: &str = "\x1b[95m" ;
pub const FG_LIGHT_CYAN   : &str = "\x1b[96m" ;
pub const FG_WHITE        : &str = "\x1b[97m" ;

pub const BG_BLACK        : &str = "\x1b[40m" ;
pub const BG_RED          : &str = "\x1b[41m" ;
pub const BG_GREEN        : &str = "\x1b[42m" ;
pub const BG_YELLOW       : &str = "\x1b[43m" ;
pub const BG_BLUE         : &str = "\x1b[44m" ;
pub const BG_MAGENTA      : &str = "\x1b[45m" ;
pub const BG_CYAN         : &str = "\x1b[46m" ;
pub const BG_LIGHT_GRAY   : &str = "\x1b[47m" ;
pub const BG_DEFAULT      : &str = "\x1b[49m" ;

pub const BG_DARK_GRAY    : &str = "\x1b[100m";
pub const BG_LIGHT_RED    : &str = "\x1b[101m";
pub const BG_LIGHT_GREEN  : &str = "\x1b[102m";
pub const BG_LIGHT_YELLOW : &str = "\x1b[103m";
pub const BG_LIGHT_BLUE   : &str = "\x1b[104m";
pub const BG_LIGHT_MAGENTA: &str = "\x1b[105m";
pub const BG_LIGHT_CYAN   : &str = "\x1b[106m";
pub const BG_WHITE        : &str = "\x1b[107m";

// pub const : &str = "\x1b[m";

// use std::fmt::{Display, Error, Formatter};

// pub struct Style<T: Display> {
//     start: &'static str,
//     stop : &'static str,
//     embeded: T,
// }

// impl<T: Display> Display for Style<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         self.start  .fmt(f)?;
//         self.embeded.fmt(f)?;
//         self.stop   .fmt(f)?;
//         Ok(())
//     }
// }

// pub fn bold<T: Display>(embeded: T) -> Style<T> {
//     Style {
//         start: "\x1b[1m",
//         stop : "\x1b[22m",
//         embeded,
//     }
// }