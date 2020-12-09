use anyhow::{Context, Result};
use crate::util::*;
use self::util::*;

/// The int size used in this challenge
///
/// I overflowed with u32 lol, I didn't expect the numbers to be that big
type Int = u64;

pub(crate) fn day9() {
    let xmas_data = XmasData::new_parse(&get_input("day9"), 25).unwrap();
    part1(&xmas_data);
    part2(&xmas_data);
}

fn part1(xmas_data: &XmasData) {
    let num = xmas_data.find_invalid_num().expect("no invalid number");

    println!("The first number that does not have the property is {}", num);
    assert_eq!(num, 731031916);
}

fn part2(xmas_data: &XmasData) {
    let weakness = xmas_data.find_encryption_weakness().expect("no weakness");

    println!(
        "The encryption weakness in your XMAS-encrypted list of numbers is {}",
        weakness
    );
    assert_eq!(weakness, 93396727);
}

/// eXchange-Masking Addition System
struct XmasData {
    data: Array<Int>,
    /// The number of previous sums to consider; also the length of the preamble
    sum_len: usize,
}

impl XmasData {
    /// Try to create a new instance by parsing `s`
    fn new_parse(s: &str, sum_len: usize) -> Result<Self> {
        s.parse_lines()
            .map(|data| Self { data, sum_len })
            .context("failed parsing number in XMAS encoding")
    }

    /// Finds and returns the first invalid number
    fn find_invalid_num(&self) -> Option<Int> {
        // We consider the last self.sum_len numbers (starting with the preamble)
        self.data.windows(self.sum_len)
            // We then operate on every number after the preamble
            .zip(self.data.iter().skip(self.sum_len))
            // Our number is the one != the sum of any 2 previous numbers
            .find_map(|(prev_nums, &num)| Some(num).filter(|_|
                !prev_nums.iter().sums_of_any_2().any(|sum| sum == num)))
    }

    /// Finds a contiguous set of at least 2 numbers that sum to `sum`
    fn find_nums_that_sum_to(&self, sum: Int) -> Option<&[Int]> {
        for i in 0..self.data.len() {
            let mut set_sum = self.data[i];
            for j in i+1..self.data.len() {
                set_sum += self.data[j];
                if set_sum == sum { return Some(&self.data[i..=j]) }
                if set_sum > sum { break }
            }
        }
        None
    }

    /// Calculates the encryption weakness for this XMAS-encoded data
    fn find_encryption_weakness(&self) -> Option<Int> {
        let invalid_num = self.find_invalid_num()?;
        let set = self.find_nums_that_sum_to(invalid_num)?;
        let min = set.iter().min()?;
        let max = set.iter().max()?;
        Some(min + max)
    }
}

mod util {
    use std::{ops::Add, slice};
    use itertools::{Itertools, Permutations};

    pub(super) trait SumsOfAny2Trait<T, U>: Iterator<Item = T> + Sized
    where
        T: Copy + Add<Output = U>,
    {
        fn sums_of_any_2(self) -> SumsOfAny2<Permutations<Self>> {
            SumsOfAny2::new(self.permutations(2))
        }
    }

    impl<'a, T, U> SumsOfAny2Trait<&'a T, U> for slice::Iter<'a, T>
    where
        &'a T: Copy + Add<Output = U>,
    {}

    pub(super) struct SumsOfAny2<I> {
        iter: I,
    }

    impl<I> SumsOfAny2<I>
    {
        fn new(iter: I) -> Self {
            Self { iter }
        }
    }

    impl<T, I, U> Iterator for SumsOfAny2<I>
    where
        T: Copy + Add<Output = U>,
        I: Iterator<Item = Vec<T>>,
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|v| v[0] + v[1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part1() {
        let xmas_data = XmasData::new_parse(INPUT, 5).unwrap();
        assert_eq!(xmas_data.find_invalid_num(), Some(127));
    }

    #[test]
    fn test_part2() {
        let xmas_data = XmasData::new_parse(INPUT, 5).unwrap();
        assert_eq!(xmas_data.find_encryption_weakness(), Some(62));
    }
}
