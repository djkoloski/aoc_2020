use std::str::FromStr;
use problem::{Problem, solve};

struct Answers(u32);

impl FromStr for Answers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Answers(s.chars().fold(0, |acc, c| acc | (1 << (c as usize - 'a' as usize)))))
    }
}

struct Day6;
impl Problem for Day6 {
    type Input = Answers;
    type Part1Output = u32;
    type Part2Output = u32;
    type Error = ();

    fn part_1(input: &Vec<Self::Input>) -> Result<Self::Part1Output, Self::Error> {
        let mut total = 0;
        let mut acc = 0u32;
        for i in input {
            if i.0 == 0 {
                total += acc.count_ones();
                acc = 0;
            } else {
                acc |= i.0;
            }
        }

        total += acc.count_ones();

        Ok(total)
    }

    fn part_2(input: &Vec<Self::Input>) -> Result<Self::Part2Output, Self::Error> {
        let mut total = 0;
        let mut acc = 0xffffffffu32;
        for i in input {
            if i.0 == 0 {
                total += acc.count_ones();
                acc = 0xffffffffu32;
            } else {
                acc &= i.0;
            }
        }

        total += acc.count_ones();

        Ok(total)
    }
}

fn main() {
    solve::<Day6>("input").unwrap();
}