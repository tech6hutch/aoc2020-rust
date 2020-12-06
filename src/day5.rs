use anyhow::Context;
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day5() {
    let mut seat_ids = get_input("day5").lines().map(|line| {
        let (row, col) = row_col_from_binary_str(line)
            .with_context(|| format!("couldn't parse pass '{}'", line))
            .unwrap();
        seat_id(row, col)
    }).collect_vec();
    seat_ids.sort();

    part1(&seat_ids);
    part2(&seat_ids);
}

fn part1(seat_ids: &Vec<u64>) {
    let highest_seat_id = seat_ids.iter().copied().max()
        .expect("no passes");

    println!("The highest seat ID is {}", highest_seat_id);
    assert_eq!(highest_seat_id, 864);
}

fn part2(seat_ids: &Vec<u64>) {
    let seat_id = seat_ids.iter().copied().tuple_windows()
        .find(|&(prev, next)| prev + 1 != next)
        .map(|(prev, _)| prev + 1)
        .expect("couldn't find your seat ID");

    println!("The ID of your seat is {}", seat_id);
    assert_eq!(seat_id, 739);
}

fn seat_id(row: u8, col: u8) -> u64 {
    let row: u64 = row.into();
    let col: u64 = col.into();
    row * 8 + col
}

fn row_col_from_binary_str(s: &str) -> Option<(u8, u8)> {
    const ROW_LEN: usize = 7;
    const COL_LEN: usize = 3;

    if !(s.len() == (ROW_LEN + COL_LEN) && s.chars().all(|c| c.is_ascii())) {
        return None;
    }

    let row_bin = s[..ROW_LEN].as_bytes();
    let col_bin = s[ROW_LEN..].as_bytes();
    if col_bin.len() != COL_LEN {
        return None;
    }
    assert_eq!(row_bin.len(), ROW_LEN);
    assert_eq!(col_bin.len(), COL_LEN);

    let row: u8 = row_bin.iter().enumerate().try_fold(0, |row, (i, c)|
        Ok(match c {
            b'F' => {
                // Keep lower half
                row
            }
            b'B' => {
                // Keep upper half
                let half_range = 1 << (ROW_LEN - 1 - i);
                row + half_range
            }
            _ => {
                // Wtf is this
                return Err(());
            }
        })
    ).ok()?;

    let col: u8 = col_bin.iter().enumerate().try_fold(0, |col, (i, c)|
        Ok(match c {
            b'L' => {
                // Keep lower half
                col
            }
            b'R' => {
                // Keep upper half
                let half_range = 1 << (COL_LEN - 1 - i);
                col + half_range
            }
            _ => {
                // Wtf is this
                return Err(());
            }
        })
    ).ok()?;

    Some((row, col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        #[derive(Debug, PartialEq)]
        struct Pass {
            s: &'static str,
            row: u8, col: u8,
            seat_id: u64,
        }
        static PASSES: &[Pass] = &[
            Pass { s: "FBFBBFFRLR", row: 44, col: 5, seat_id: 357 },
            Pass { s: "BFFFBBFRRR", row: 70, col: 7, seat_id: 567 },
            Pass { s: "FFFBBBFRRR", row: 14, col: 7, seat_id: 119 },
            Pass { s: "BBFFBBFRLL", row: 102, col: 4, seat_id: 820 },
        ];

        for expected_pass in PASSES {
            let s = expected_pass.s;
            let (row, col) = row_col_from_binary_str(s)
                .expect("failed to parse row & col");
            let seat_id = seat_id(row, col);
            assert_eq!(expected_pass, &Pass { s, row, col, seat_id });
        }
    }
}
