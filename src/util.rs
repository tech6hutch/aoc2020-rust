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

pub(crate) trait ParseStrLines<T: FromStr> {
    fn parse_lines(self) -> Result<Vec<T>, T::Err>;
}

impl<T: FromStr> ParseStrLines<T> for &str {
    fn parse_lines(self) -> Result<Vec<T>, T::Err> {
        self
            .lines()
            .map(|s| s.parse())
            .collect()
    }
}
