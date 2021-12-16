use crate::sbi::console_putchar;
use core::fmt::{self,Write};

struct MyStdout;

impl Write for MyStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::logging::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    MyStdout.write_fmt(with_color!(args, color_code)).unwrap();
}

pub fn print(args: fmt::Arguments) {
    MyStdout.write_fmt(args).unwrap();
}








//logging functionality



pub fn get_max_level() -> Level {
    match option_env!("LOG") {
        Some("error") => Level::Error,
        Some("warn") => Level::Warn,
        Some("info") => Level::Info,
        Some("debug") => Level::Debug,
        Some("trace") => Level::Trace,
        _ => Level::Off,
    }
}

#[derive(Copy, Clone)]
pub enum Level {
    Off = 0,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Level::Error => write!(f, "ERROR"),
           Level::Warn => write!(f, "WARN"),
           Level::Info => write!(f, "INFO"),
           Level::Debug => write!(f, "DEBUG"),
           Level::Trace => write!(f, "TRACE"),
           Level::Off => write!(f, "OFF"),
       }
    }
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
        _ => 0,
    }
}

pub fn log(level:&Level, args: fmt::Arguments) {
    print_in_color(
        format_args!(
            "[{:>5}] {}\n",
            *level,
            args
        ),
        level_to_color_code(*level),
    );
}

#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = &$lvl;
        if *lvl as usize <= $crate::logging::get_max_level() as usize {
            $crate::logging::log(
                lvl,
                __log_format_args!($($arg)+),
            );
        }
    });
}

macro_rules! info {
    ($($arg:tt)+) => (
        log!($crate::logging::Level::Info, $($arg)+)
    )
}

macro_rules! warn {
    ($($arg:tt)+) => (
        log!($crate::logging::Level::Warn, $($arg)+)
    )
}

macro_rules! error {
    ($($arg:tt)+) => (
        log!($crate::logging::Level::Error, $($arg)+)
    )
}

macro_rules! debug {
    ($($arg:tt)+) => (
        log!($crate::logging::Level::Debug, $($arg)+)
    )
}

macro_rules! trace {
    ($($arg:tt)+) => (
        log!($crate::logging::Level::Trace, $($arg)+)
    )
}