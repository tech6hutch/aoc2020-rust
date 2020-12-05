use std::{str::FromStr, fs};
use anyhow::Context;

pub(crate) fn get_input(name: &str) -> String {
    fs::read_to_string(format!("./inputs/{}.txt", name))
        .with_context(|| format!("no input file with name {}.txt", name))
        .unwrap()
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
