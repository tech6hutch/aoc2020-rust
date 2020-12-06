use std::{str::FromStr, collections::HashSet};
use anyhow::{Error, Result, bail};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day6() {
    let all_group_answers: AllGroupsAnswers = get_input("day6").parse().unwrap();
    part1(&all_group_answers);
    part2(&all_group_answers);
}

fn part1(all_group_answers: &AllGroupsAnswers) {
    let sum_of_unique_yeses = all_group_answers.count_yeses_any();

    println!(
        "The sum of the number of questions to which anyone answered yes in each group is {}",
        sum_of_unique_yeses
    );
    assert_eq!(sum_of_unique_yeses, 6662);
}

fn part2(all_group_answers: &AllGroupsAnswers) {
    let sum_of_yeses_all = all_group_answers.count_yeses_all();

    println!(
        "The sum of the number of questions to which everyone answered yes in each group is {}",
        sum_of_yeses_all
    );
    assert_eq!(sum_of_yeses_all, 3382);
}

struct AllGroupsAnswers(Vec<GroupAnswers>);

impl AllGroupsAnswers {
    fn count_yeses_any(&self) -> usize {
        self.0.iter().map(|g| g.count_yeses_any()).sum()
    }

    fn count_yeses_all(&self) -> usize {
        self.0.iter().map(|g| g.count_yeses_all()).sum()
    }
}

impl FromStr for AllGroupsAnswers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n\n")
            .map(str::parse)
            .try_collect()
            .map(AllGroupsAnswers)
    }
}

struct GroupAnswers {
    any: HashSet<char>,
    all: HashSet<char>,
}

impl GroupAnswers {
    fn count_yeses_any(&self) -> usize {
        self.any.iter().count()
    }

    fn count_yeses_all(&self) -> usize {
        self.all.iter().count()
    }
}

impl FromStr for GroupAnswers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn letter_or_err(c: char) -> Result<char> {
            if !c.is_ascii_alphabetic() { bail!("non-letter found") }
            Ok(c)
        }

        fn all_letters_or_err(s: &str) -> Result<&str> {
            s.chars()
                .map(letter_or_err)
                // I don't want the chars
                .map_results(|_| ())
                .try_collect()
                .map(|()| s)
        }

        let any = s.chars()
            .filter(|&c| c != '\n')
            .map(letter_or_err)
            .try_collect()?;

        let all = s.lines()
            .map(all_letters_or_err)
            .map_results(|line| line.chars().collect::<HashSet<char>>())
            .fold1(|set, line| Ok(
                set?.intersection(&line?).copied().collect()
            ))
            .transpose()?
            .unwrap_or_default();

        Ok(Self { any, all })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        let sum_of_unique_yeses = INPUT
            .parse::<AllGroupsAnswers>()
            .unwrap()
            .count_yeses_any();
        assert_eq!(sum_of_unique_yeses, 11);
    }

    #[test]
    fn test_part2() {
        let sum_of_yeses_all = INPUT
            .parse::<AllGroupsAnswers>()
            .unwrap()
            .count_yeses_all();
        assert_eq!(sum_of_yeses_all, 6);
    }
}
