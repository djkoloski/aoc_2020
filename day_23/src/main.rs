use std::{collections::LinkedList, io, iter, num};

use num::ParseIntError;
use problem::{Problem, ProblemInput, solve};

struct Input {
    cups: Vec<u32>,
}

#[derive(Debug)]
enum ParseInputError {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
}

impl From<io::Error> for ParseInputError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<num::ParseIntError> for ParseInputError {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl ProblemInput for Input {
    type Error = ParseInputError;
    fn parse<R: io::BufRead>(mut reader: R) -> Result<Self, Self::Error> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        Ok(Self {
            cups: line.chars().map(|c| c as u32 - '1' as u32).collect(),
        })
    }
}

struct Day23;
impl Problem for Day23 {
    type Input = Input;
    type Part1Output = String;
    type Part2Output = u64;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut next_cup = vec![0; input.cups.len()];
        let cups = input.cups.iter().cloned();
        for (cup, next) in cups.clone().zip(cups.cycle().skip(1)) {
            next_cup[cup as usize] = next;
        }

        let mut current = input.cups[0];
        for _ in 0..100 {
            let mut target = (current + next_cup.len() as u32 - 1) % next_cup.len() as u32;

            let h0 = next_cup[current as usize];
            let h1 = next_cup[h0 as usize];
            let h2 = next_cup[h1 as usize];

            next_cup[current as usize] = next_cup[h2 as usize];

            let hand = [h0, h1, h2];

            while hand.contains(&target) {
                target = (target + next_cup.len() as u32 - 1) % next_cup.len() as u32;
            }

            next_cup[h2 as usize] = next_cup[target as usize];
            next_cup[target as usize] = h0;

            current = next_cup[current as usize];
        }

        let mut cups = vec![0; next_cup.len()];
        let mut current = 0;
        for i in 0..cups.len() {
            cups[i] = next_cup[current as usize];
            current = next_cup[current as usize];
        }
        Ok(cups.iter().cycle().skip_while(|&x| *x != 0).skip(1).take(cups.len() - 1).map(|x| ('1' as u8 + *x as u8) as char).collect())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut next_cup = vec![0; 1_000_000];
        let cups = input.cups.iter().cloned().chain(input.cups.len() as u32..next_cup.len() as u32);
        for (cup, next) in cups.clone().zip(cups.cycle().skip(1)) {
            next_cup[cup as usize] = next;
        }

        let mut current = input.cups[0];
        for _ in 0..10_000_000 {
            let mut target = (current + next_cup.len() as u32 - 1) % next_cup.len() as u32;

            let h0 = next_cup[current as usize];
            let h1 = next_cup[h0 as usize];
            let h2 = next_cup[h1 as usize];

            next_cup[current as usize] = next_cup[h2 as usize];

            let hand = [h0, h1, h2];

            while hand.contains(&target) {
                target = (target + next_cup.len() as u32 - 1) % next_cup.len() as u32;
            }

            next_cup[h2 as usize] = next_cup[target as usize];
            next_cup[target as usize] = h0;

            current = next_cup[current as usize];
        }

        let a0 = next_cup[0] as u64;
        let a1 = next_cup[a0 as usize] as u64;
        Ok((a0 + 1) * (a1 + 1))
    }
}

fn main() {
    solve::<Day23>("input").unwrap();
}
