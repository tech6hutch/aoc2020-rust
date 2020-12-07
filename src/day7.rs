use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroUsize,
    str::FromStr,
};
use anyhow::{Context, Error, Result};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day7() {
    let rules = get_input("day7").parse().unwrap();
    part1(&rules);
    part2(&rules);
}

fn part1(rules: &RuleSet) {
    let count = rules.count_bags_that_can_contain(&BagDesc::new("shiny gold"));

    println!("{} bag colors can eventually contain at least one shiny gold bag", count);
    assert_eq!(count, 278);
}

fn part2(rules: &RuleSet) {
    let count = rules.count_required_bags_for(&BagDesc::new("shiny gold"));

    println!("{} individual bags are required inside your single shiny gold bag", count);
    assert_eq!(count, 45157);
}

struct RuleSet(HashMap<BagDesc, Rule>);

impl RuleSet {
    fn count_bags_that_can_contain(&self, bag: &BagDesc) -> usize {
        fn can_contain(map: &HashMap<BagDesc, Rule>, needle: &BagDesc, haystack: &BagDesc) -> bool {
            assert_ne!(needle, haystack);

            let rule = if let Some(rule) = map.get(haystack) { rule }
                else { return false; };

            rule.contains.iter().any(|BagDescCount { bag: haystack, .. }|
                haystack == needle || can_contain(map, needle, haystack))
        }

        let map = &self.0;

        map.keys()
            .filter(|&top_level_bag|
                // Skip the bag we're searching for, ofc
                top_level_bag != bag &&
                // Check if this bag can contain the bag we're searching for
                can_contain(map, bag, top_level_bag))
            .count()
    }

    /// Counts the bags required to be contained inside `bag`
    fn count_required_bags_for(&self, bag: &BagDesc) -> usize {
        /// Counts the required bags for `bag`, recursively
        fn count_contained_bags(
            rules: &HashMap<BagDesc, Rule>, bag: &BagDesc
        ) -> usize {
            let rule = unwrap_or!(rules.get(bag), return 0);

            rule.contains.iter()
                .map(|BagDescCount { bag, count }| {
                    let count = count.get();

                    let contained_bags = count_contained_bags(rules, bag);

                    // There are `count` bags of this kind, so there are `count * contained_bags`
                    // in them, so there are `count + count * contained_bags` bags total, or,
                    // `count * (1 + contained_bags)`.
                    count * (1 + contained_bags)
                })
                .sum()
        }

        count_contained_bags(&self.0, bag)
    }
}

impl FromStr for RuleSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.lines().map(|line| {
            const CONTAIN: &str = "contain";

            let contain_idx = line.find(CONTAIN)
                .with_context(|| format!("failed to parse rule '{}'", line))?;

            let outer_desc: BagDesc = line[..contain_idx].trim_end()
                .parse()
                .context("failed to parse rule")?;

            let rule: Rule = line[(contain_idx+CONTAIN.len())..]
                .trim_start()
                .strip_suffix(".")
                .with_context(|| format!("failed to parse '.' in rule '{}'", line))?
                .parse()?;

            Ok((outer_desc, rule))
        }).try_collect().map(Self)
    }
}

struct Rule {
    contains: Box<[BagDescCount]>,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if let Some(bdc) = self.contains.first() {
            write!(f, "{}", bdc)?;
        } else {
            write!(f, "no other bags")?;
        }
        for bdc in self.contains.iter().skip(1) {
            write!(f, ", {}", bdc)?;
        }
        Ok(())
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let contains = if s == "no other bags" {
            Default::default()
        } else {
            s.split(",")
                .map(str::trim_start)
                .map(str::parse)
                .try_collect()
                .context("failed to parse rule")?
        };

        Ok(Self { contains })
    }
}

struct BagDescCount {
    count: NonZeroUsize,
    bag: BagDesc,
}

impl Display for BagDescCount {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{} {} bag{}",
            self.count, self.bag, if self.count.get() == 1 { "" } else { "s" }
        )
    }
}

impl FromStr for BagDescCount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let non_digit_idx = s
            .find(|c: char| !c.is_ascii_digit())
            .with_context(|| format!("found no number in '{}'", s))?;

        let count = s[..non_digit_idx]
            .parse()
            .with_context(|| format!("couldn't parse number from '{}'", s))?;

        let bag_desc = s[non_digit_idx..].trim_start()
            .parse()
            .with_context(|| format!("couldn't parse bag description from '{}'", s))?;

        Ok(Self { count, bag: bag_desc })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct BagDesc(Box<str>);

impl BagDesc {
    fn new(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl Display for BagDesc {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl FromStr for BagDesc {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.strip_suffix("bags")
            .or_else(|| s.strip_suffix("bag"))
            .with_context(|| format!(
                "expected bag description to end with 'bag(s)' but it did not: '{}'", s
            ))?
            .trim_end();

        Ok(Self(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_part1() {
        let rules: RuleSet = INPUT.parse().unwrap();
        let count = rules.count_bags_that_can_contain(&BagDesc::new("shiny gold"));
        assert_eq!(count, 4);
    }

    #[test]
    fn test_part2_ex1() {
        let rules: RuleSet = INPUT.parse().unwrap();
        let count = rules.count_required_bags_for(&BagDesc::new("shiny gold"));
        assert_eq!(count, 32);
    }

    #[test]
    fn test_part2_ex2() {
        static INPUT: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let rules: RuleSet = INPUT.parse().unwrap();
        let count = rules.count_required_bags_for(&BagDesc::new("shiny gold"));
        assert_eq!(count, 126);
    }
}
