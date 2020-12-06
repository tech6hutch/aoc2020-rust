use std::{collections::HashMap, convert::{TryFrom, TryInto}, ops::RangeInclusive};
use anyhow::{Context, Error, Result, bail};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day4() {
    let input = get_input("day4");
    let passports = input.as_str().try_into().unwrap();
    part1(&passports);
    part2(&passports);
}

fn part1(passports: &Passports) {
    let valid = passports.0.iter()
        .filter(|p| p.is_valid_part1())
        .count();

    println!("{} passports are valid", valid);
    assert_eq!(valid, 260);
}

fn part2(passports: &Passports) {
    let valid = passports.0.iter()
        .filter(|p| p.is_valid_part2())
        .count();

    println!("{} passports are valid", valid);
    assert_eq!(valid, 153);
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

    fn is_valid_part1(&self) -> bool {
        Self::REQUIRED_FIELDS.iter()
            .all(|field| self.0.contains_key(field))
    }

    fn is_valid_part2(&self) -> bool {
        if !self.is_valid_part1() { return false; }

        self.validate().is_ok()
    }

    // I added error messages to help diagnose a bug...but in the process I
    // fixed the bug anyway ¯\_(ツ)_/¯
    fn validate(&self) -> Result<()> {
        if !self.is_valid_part1() {
            bail!("missing field(s)");
        }

        let Self(map) = self;
        static FIELD_RANGES: &[(&str, RangeInclusive<i32>)] = &[
            ("byr", 1920..=2002),
            ("iyr", 2010..=2020),
            ("eyr", 2020..=2030),
        ];
        for (field, range) in FIELD_RANGES {
            let value: i32 = map[field].parse()
                .with_context(|| format!("failed parsing {}", field))?;
            if !range.contains(&value) {
                bail!("{} not in range", value);
            }
        }

        fn validate_hgt(hgt: &str, range: RangeInclusive<i32>) -> Result<()> {
            hgt.parse::<i32>()
                .context("failed parsing hgt")
                .and_then(|hgt|
                    if range.contains(&hgt) { Ok(()) }
                    else { bail!("hgt not in range") })
        }
        let hgt = map["hgt"];
        if let Some(hgt) = hgt.strip_suffix("cm") {
            validate_hgt(hgt, 150..=193)?;
        } else if let Some(hgt) = hgt.strip_suffix("in") {
            validate_hgt(hgt, 59..=76)?;
        } else {
            bail!("hgt not an integer ending with 'cm' or 'in'");
        }

        map["hcl"].strip_prefix("#")
            .context("hcl must start with '#'")
            .and_then(|hcl| i32::from_str_radix(hcl, 16)
                .context("hcl must be valid hexadecimal"))?;

        if !matches!(
            map["ecl"],
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ) {
            bail!("ecl not one of valid options");
        }

        let pid = map["pid"];
        if pid.len() != 9 { bail!("pid not 9 digits"); }
        pid.parse::<u32>().context("pid must be a valid int")?;

        Ok(())
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

    #[test]
    fn test_part1() {
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
        let passports: Passports = INPUT.try_into().unwrap();
        let valid = passports.0.iter()
            .filter(|p| p.is_valid_part1())
            .count();
        assert_eq!(valid, 2);
    }

    #[test]
    fn test_part2() {
        static INVALID: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let passports: Passports = INVALID.try_into().unwrap();
        for (i, p) in passports.0.iter().enumerate() {
            if p.is_valid_part2() {
                panic!("invalid passport {} of {} was marked valid",
                    i, passports.0.len());
            }
        }

        static VALID: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports: Passports = VALID.try_into().unwrap();
        for (i, p) in passports.0.iter().enumerate() {
            if !p.is_valid_part2() {
                panic!("valid passport {} of {} was marked invalid",
                    i, passports.0.len());
            }
        }
    }
}
