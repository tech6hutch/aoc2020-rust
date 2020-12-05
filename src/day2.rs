use std::{fmt::Debug, str::FromStr};
use anyhow::{Context, Error};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day2() {
    let input = get_input("day2");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let answer = count_valid_passwords::<old_job::Policy>(
        &input.parse_lines().unwrap()
    );

    println!("According to their policies, {} passwords are valid", answer);
    assert_eq!(answer, 469);
}

fn part2(input: &str) {
    let answer = count_valid_passwords::<north_pole_toboggan_rental_shop::Policy>(
        &input.parse_lines().unwrap()
    );

    println!("According to their policies, {} passwords are valid", answer);
    assert_eq!(answer, 267);
}

#[derive(Debug)]
struct PasswordWithPolicy<P> {
    policy: P,
    passwd: Box<str>,
}

impl<P: PasswordPolicy> PasswordWithPolicy<P> {
    fn is_valid(&self) -> bool {
        self.policy.validate(&self.passwd)
    }
}

impl<P: FromStr<Err = Error>> FromStr for PasswordWithPolicy<P> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SEP: &'static str = ": ";
        const ERR: &str = "invalid str";

        let sep_idx = s.find(SEP).context(ERR)?;

        let policy = s[..sep_idx].parse()?;
        let passwd = s[sep_idx+SEP.len()..].into();

        Ok(Self { policy, passwd })
    }
}

trait PasswordPolicy: Debug {
    fn validate(&self, passwd: &str) -> bool;
}

fn count_valid_passwords<P: PasswordPolicy>(
    passwords: &Vec<PasswordWithPolicy<P>>
) -> usize {
    passwords.iter().filter(|pp| pp.is_valid()).count()
}

mod north_pole_toboggan_rental_shop {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub(super) struct Policy {
        letter: char,
        positions: [usize; 2],
    }

    impl PasswordPolicy for Policy {
        fn validate(&self, passwd: &str) -> bool {
            let Self { letter, positions } = *self;
            positions.iter().copied()
                .filter(|pos| letter == passwd.chars().nth(pos - 1)
                    .expect("position not found in password"))
                .count() == 1
        }
    }

    impl FromStr for Policy {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            const HYPHEN: &str = "-";
            const SPACE: &str = " ";
            const ERR: &str = "invalid policy";

            (|| -> anyhow::Result<Self> {
                let hyphen_idx = s.find(HYPHEN)
                    .context("no hyphen")?;
                let space_idx = s.find(SPACE)
                    .context("no space")?;

                let pos1 = s[..hyphen_idx].parse()
                    .context("invalid 1st position")?;
                let pos2 = s[hyphen_idx+HYPHEN.len()..space_idx].parse()
                    .context("invalid 2nd position")?;
                let letter = s[space_idx+SPACE.len()..].chars().exactly_one()
                    .map_err(error_from_debug)
                    .context("expected a single char")?;

                Ok(Self { positions: [pos1, pos2], letter })
            })()
            .context(ERR)
        }
    }
}

mod old_job {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub(super) struct Policy {
        letter: char,
        min: usize,
        max: usize,
    }

    impl PasswordPolicy for Policy {
        fn validate(&self, passwd: &str) -> bool {
            let Self { min, max, letter } = *self;
            (min..=max).contains(
                &passwd.chars().filter(|c| *c == letter).count()
            )
        }
    }

    impl FromStr for Policy {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            const HYPHEN: &str = "-";
            const SPACE: &str = " ";
            const ERR: &str = "invalid policy";

            (|| -> anyhow::Result<Self> {
                let hyphen_idx = s.find(HYPHEN)
                    .context("no hyphen")?;
                let space_idx = s.find(SPACE)
                    .context("no space")?;

                let min = s[..hyphen_idx].parse()
                    .context("invalid min")?;
                let max = s[hyphen_idx+HYPHEN.len()..space_idx].parse()
                    .context("invalid max")?;
                let letter = s[space_idx+SPACE.len()..].chars().exactly_one()
                    .map_err(|e| Error::msg(format!("{:?}", e)))
                    .context("expected a single char")?;

                Ok(Self { min, max, letter })
            })()
            .context(ERR)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        assert_eq!(
            count_valid_passwords::<old_job::Policy>(
                &TEST_INPUT.parse_lines().unwrap()
            ),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            count_valid_passwords::<north_pole_toboggan_rental_shop::Policy>(
                &TEST_INPUT.parse_lines().unwrap()
            ),
            1
        );
    }
}
