use std::str::FromStr;
use crate::util::*;

pub(crate) fn day1() {
    let report = get_input("day1").parse().unwrap();
    part1(&report);
    part2(&report);
}

fn part1(report: &ExpenseReport) {
    let answer = product_of_2_entries_with_sum_2020(
        report
    ).unwrap();

    println!("The product of the two entries that sum to 2020 is {}", answer);
    assert_eq!(answer, 888331);
}

fn part2(report: &ExpenseReport) {
    let answer = product_of_3_entries_with_sum_2020(
        report
    ).unwrap();

    println!("The product of the three entries that sum to 2020 is {}", answer);
    assert_eq!(answer, 130933530);
}

fn product_of_2_entries_with_sum_2020(ExpenseReport(entries): &ExpenseReport)
-> Result<i32, &'static str> {
    entries.iter().copied().find_map(|n1|
        entries.iter().copied().find(|n2| n1 + n2 == 2020)
            .map(|n2| n1 * n2))
        .ok_or("couldn't find any entries that sum to 2020")
}

fn product_of_3_entries_with_sum_2020(ExpenseReport(entries): &ExpenseReport)
-> Result<i32, &'static str> {
    entries.iter().copied()
        .find_map(|n1| entries.iter().copied()
            .find_map(|n2| entries.iter().copied()
                .find(|n3| n1 + n2 + n3 == 2020).map(|n3| (n2, n3)))
            .map(|(n2, n3)| n1 * n2 * n3))
        .ok_or("couldn't find any entries that sum to 2020")
}

#[derive(Clone, Debug)]
struct ExpenseReport(Vec<i32>);

impl FromStr for ExpenseReport {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse_lines()
            .map(ExpenseReport)
            .map_err(|_| "couldn't parse number")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "\
1721
979
366
299
675
1456";

    fn test_entries() -> ExpenseReport {
        TEST_INPUT.parse().unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(product_of_2_entries_with_sum_2020(&test_entries()), Ok(514579));
    }

    #[test]
    fn test_part2() {
        assert_eq!(product_of_3_entries_with_sum_2020(&test_entries()), Ok(241861950));
    }
}
