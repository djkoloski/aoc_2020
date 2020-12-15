use std::fmt;
use problem::{Problem, solve};
use smallbitvec::SmallBitVec;

fn solve_2(values: &[i32], target: i32) -> Option<(i32, i32)> {
    let half = target / 2 + 1;
    let mut bits = SmallBitVec::from_elem(half as usize, false);
    for &value in values.iter() {
        let index = if value < half { value } else { target - value };
        if index >= 0 {
            if bits[index as usize] {
                return Some((index, target - index))
            } else {
                bits.set(index as usize, true);
            }
        }
    }
    None
}

struct Solution<T>(T);

impl<T: AsRef<[i32]>> fmt::Display for Solution<T> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut product = 1;
        for &v in self.0.as_ref().iter() {
            product *= v;
        }
        write!(f, "{}", product)?;
        for (i, &v) in self.0.as_ref().iter().enumerate() {
            if i == 0 {
                write!(f, " = {}", v)?;
            } else {
                write!(f, " * {}", v)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Error {
    NoSolution,
}

const TARGET: i32 = 2020;

struct Day1;
impl Problem for Day1 {
    type Input = Vec<i32>;
    type Part1Output = Solution<[i32; 2]>;
    type Part2Output = Solution<[i32; 3]>;
    type Error = Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let (a, b) = solve_2(input.as_slice(), TARGET).ok_or(Error::NoSolution)?;
        Ok(Solution([a, b]))
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        for (i, &v) in input.iter().enumerate() {
            if let Some((a, b)) = solve_2(&input[i + 1..], TARGET - v) {
                return Ok(Solution([v, a, b]));
            }
        }
        Err(Error::NoSolution)
    }
}

fn main() {
    solve::<Day1>("input").unwrap();
}
