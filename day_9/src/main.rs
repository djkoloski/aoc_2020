use std::ops::Range;
use problem::{Problem, solve};
use smallbitvec::SmallBitVec;

fn solve_2(values: &[i64], target: i64) -> Option<(i64, i64)> {
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

struct Day9;
impl Problem for Day9 {
    type Input = Vec<i64>;
    type Part1Output = i64;
    type Part2Output = i64;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        const PREAMBLE_LEN: usize = 25;

        for i in PREAMBLE_LEN..input.len() {
            let values = &input[i - PREAMBLE_LEN..i];
            let target = input[i];
            if let None = solve_2(values, target) {
                return Ok(target);
            }
        }

        Err(())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let target = Self::part_1(input)?;

        let mut range = Range {
            start: 0,
            end: 0,
        };

        let mut total = 0;
        while range.start < input.len() && range.end <= input.len() {
            if total < target {
                total += input[range.end];
                range.end += 1;
            } else if total > target {
                total -= input[range.start];
                range.start += 1;
            } else {
                let range = &input[range];
                return Ok(range.iter().min().unwrap() + range.iter().max().unwrap());
            }
        }

        Err(())
    }
}

fn main() {
    solve::<Day9>("input").unwrap();
}
