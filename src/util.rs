use std::{fmt::Debug, fs, str::FromStr};
use anyhow::{Context, Error};

pub(crate) fn get_input(name: &str) -> String {
    fs::read_to_string(format!("./inputs/{}.txt", name))
        .with_context(|| format!("no input file with name {}.txt", name))
        .unwrap()
}

/// Converts an error to an `anyhow`-compatible error using its Debug impl.
///
/// Useful for "errors" that don't actually implement `std::error::Error`, since
/// they can't be converted to `anyhow::Error` as easily.
pub(crate) fn error_from_debug(e: impl Debug) -> Error {
    Error::msg(format!("{:?}", e))
}

// TODO: I *think* this won't break past days' solutions
pub(crate) trait ParseStrLines<Coll, E> {
    fn parse_lines(self) -> Result<Coll, E>;
}

impl<T: FromStr> ParseStrLines<Vec<T>, T::Err> for &str {
    fn parse_lines(self) -> Result<Vec<T>, T::Err> {
        self
            .lines()
            .map(str::parse)
            .collect()
    }
}

impl<T: FromStr> ParseStrLines<Array<T>, T::Err> for &str {
    fn parse_lines(self) -> Result<Array<T>, T::Err> {
        self
            .lines()
            .map(str::parse)
            .collect()
    }
}

// I always forget the brackets
pub(crate) type Array<T> = Box<[T]>;

macro_rules! unwrap_or {
    ($opt:expr, $default:expr) => {
        if let Some(t) = $opt { t }
        else { $default }
    };
    ($res:expr, $e:ident => $default:expr) => {
        match $res {
            Ok(t) => t,
            Err($e) => $default,
        }
    };
}
