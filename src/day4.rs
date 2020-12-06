use std::{collections::HashMap, convert::{TryFrom, TryInto}};
use anyhow::{Context, Error, Result};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day4() {
    let input = get_input("day4");
    part1(&input);
}

fn part1(input: &str) {
    let passports: Passports = input.try_into().unwrap();
    let valid = passports.0.iter()
        .filter(|p| p.is_valid())
        .count();

    println!("{} passports are valid", valid);
    assert_eq!(valid, 260);
}

struct Passports<'a>(Box<[Passport<'a>]>);

impl<'a> TryFrom<&'a str> for Passports<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s
            .split("\n\n")
            .map(Passport::try_from)
            .try_collect()
            .map(Passports)
    }
}

struct Passport<'a>(HashMap<&'a str, &'a str>);

impl<'a> Passport<'a> {
    const REQUIRED_FIELDS: &'static [&'static str] = &[
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ];

    fn is_valid(&self) -> bool {
        Self::REQUIRED_FIELDS.iter()
            .all(|field| self.0.contains_key(field))
    }
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s
            .split(&[' ', '\n'][..])
            .filter(|kv| kv.len() > 0)
            .map(|kv| kv
                .split(':')
                .collect_tuple::<(&str, &str)>()
                .with_context(|| format!("invalid key:value pair '{}'", kv)))
            .try_collect()
            .map(Passport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_part1() {
        let passports: Passports = INPUT.try_into().unwrap();
        let valid = passports.0.iter()
            .filter(|p| p.is_valid())
            .count();
        assert_eq!(valid, 2);
    }
}
