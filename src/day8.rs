use std::{collections::HashSet, convert::TryFrom, str::FromStr};
use anyhow::{Context, Error, Result, bail};
use itertools::Itertools;
use crate::util::*;

pub(crate) fn day8() {
    let program = get_input("day8").parse().unwrap();
    part1(&program);
    part2(&program);
}

fn part1(program: &Program) {
    let acc = program.run_until_loop();

    println!(
        "The value in the accumulator, immediately before any instruction would be executed a second time, is {}",
        acc
    );
    assert_eq!(acc, 1801);
}

fn part2(program: &Program) {
    let acc = program.run_fixing_loop();

    println!(
        "The value in the accumulator after the fixed program terminates is {}",
        acc
    );
    assert_eq!(acc, 2060);
}

#[derive(Clone, Debug)]
struct Program {
    instrs: Array<Instr>,
}

impl Program {
    fn new(instrs: Array<Instr>) -> Self {
        Self { instrs }
    }

    /// Runs the program, but stops before an instruction would be executed twice.
    /// Returns the value of the accumulator at that point.
    fn run_until_loop(&self) -> i32 {
        match self.try_run_until_loop() {
            Ok(acc) => acc,
            Err(acc) => acc,
        }
    }

    /// Runs the program, but stops if an instruction would be executed twice.
    /// Returns the value of the accumulator at that point, in the Ok or Err.
    fn try_run_until_loop(&self) -> Result<i32, i32> {
        let mut acc = 0;
        let mut i = 0;
        let mut seen_indices = HashSet::new();
        while let Some(&Instr { op, arg }) = self.instrs.get(i) {
            if !seen_indices.insert(i) {
                return Err(acc);
            }

            use Op::*;
            match op {
                Acc => {
                    acc += arg;
                }
                Jmp => {
                    i = usize::try_from(
                        i64::try_from(i).unwrap() + i64::from(arg)
                    ).expect("tried to jmp to before start of program");
                    continue;
                }
                Nop => {}
            }

            i += 1;
        }
        Ok(acc)
    }

    /// Runs the program, trying to fix any one instruction that causes an infinite loop.
    /// Returns the value of the accumulator after the program terminates successfully.
    fn run_fixing_loop(&self) -> i32 {
        if let Ok(acc) = self.try_run_until_loop() {
            return acc;
        }

        let jmp_and_nop_indices = self.instrs.iter()
            .positions(|instr| matches!(instr.op, Op::Jmp | Op::Nop));
        for change_idx in jmp_and_nop_indices {
            let mut copy = self.clone();
            let op = &mut copy.instrs[change_idx].op;
            *op = match op {
                Op::Jmp => Op::Nop,
                Op::Nop => Op::Jmp,
                _ => unreachable!("we only included these ops")
            };
            if let Ok(acc) = copy.try_run_until_loop() {
                return acc;
            }
        }

        panic!("no possible change of jmp->nop or nop->jmp can fix this program");
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.parse_lines().map(Self::new)
    }
}

#[derive(Copy, Clone, Debug)]
struct Instr {
    op: Op,
    arg: i32,
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (op, arg) = s.splitn(2, ' ').collect_tuple()
            .with_context(|| format!("expected 'op arg' but found {}", s))?;
        let op = op.parse().context("failed to parse op")?;
        let arg = arg.parse().context("failed to parse arg")?;
        Ok(Self { op, arg })
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use Op::*;

        Ok(match s {
            "acc" => Acc,
            "jmp" => Jmp,
            "nop" => Nop,
            _ => bail!("unknown operation")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1() {
        let program: Program = INPUT.parse().unwrap();
        let acc = program.run_until_loop();
        assert_eq!(acc, 5);
    }

    #[test]
    fn test_part2() {
        let program: Program = INPUT.parse().unwrap();
        let acc = program.run_fixing_loop();
        assert_eq!(acc, 8);
    }
}
