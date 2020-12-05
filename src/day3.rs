use std::{iter::FromIterator, convert::{TryFrom, TryInto}, ops::Index, str::FromStr};
use anyhow::{Context, Error, Result};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day3() {
    let map: Map = get_input("day3").parse().unwrap();
    part1(&map);
    part2(&map);
}

fn part1(map: &Map) {
    let trees = map.count_trees_in_slope(3, 1);

    println!("A slope of right 3 and down 1 would encounter {} trees", trees);
    assert_eq!(trees, 259);
}

fn part2(map: &Map) {
    let product: usize = PART2_SLOPES.iter()
        .map(|&(right, down)| map.count_trees_in_slope(right, down))
        .product();

    println!("The product of the trees encountered on each slope is {}", product);
    assert_eq!(product, 2224913600);
}

static PART2_SLOPES: &[(usize, usize)] = &[
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

struct SquareRows(Box<[Square]>);

impl SquareRows {
    fn wrap_index(&self, index: usize) -> usize {
        index % self.0.len()
    }
}

impl Index<usize> for SquareRows {
    type Output = Square;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[self.wrap_index(index)]
    }
}

impl FromIterator<Square> for SquareRows {
    fn from_iter<T: IntoIterator<Item = Square>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

struct Map(Box<[SquareRows]>);

impl Map {
    fn count_trees_in_slope(&self, right: usize, down: usize) -> usize {
        self.0.iter().step_by(down).enumerate()
            .filter(|(i, row)| row[right * i] == Square::Tree)
            .count()
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.chars()
                .map(Square::try_from)
                .collect::<Result<_>>())
            .collect::<Result<_>>()
            .map(Map)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Square { Open, Tree }

impl TryFrom<char> for Square {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '.' => Self::Open,
            '#' => Self::Tree,
            _ => return Err(Error::msg("unknown square"))
        })
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().exactly_one()
            .map_err(error_from_debug)
            .context("squares are only one char")?
            .try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        let map: Map = TEST_INPUT.parse().unwrap();
        let trees = map.count_trees_in_slope(3, 1);
        assert_eq!(trees, 7);
    }

    #[test]
    fn test_part2() {
        let map: Map = TEST_INPUT.parse().unwrap();
        let product: usize = PART2_SLOPES.iter()
            .map(|&(right, down)| map.count_trees_in_slope(right, down))
            .product();
        assert_eq!(product, 336);
    }
}
