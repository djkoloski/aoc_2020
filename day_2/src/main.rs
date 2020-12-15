use std::{str::FromStr, num::ParseIntError};
use problem::{Problem, solve};

struct Input {
    min_letter: u32,
    max_letter: u32,
    letter: char,
    password: String,
}

impl Input {
    fn is_valid(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }
        count >= self.min_letter && count <= self.max_letter
    }

    fn is_valid_2(&self) -> bool {
        let min_matches = self.password.chars().nth(self.min_letter as usize - 1).unwrap() == self.letter;
        let max_matches = self.password.chars().nth(self.max_letter as usize - 1).unwrap() == self.letter;
        min_matches != max_matches
    }
}

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');

        let mut range = pieces.next().unwrap().split('-');
        let min_letter = range.next().unwrap().parse()?;
        let max_letter = range.next().unwrap().parse()?;

        let letter = pieces.next().unwrap().chars().next().unwrap();

        let password = pieces.next().unwrap().to_string();

        Ok(Input {
            min_letter,
            max_letter,
            letter,
            password,
        })
    }
}

#[derive(Debug)]
enum Error {}

struct Day2;
impl Problem for Day2 {
    type Input = Vec<Input>;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        Ok(input.iter().filter(|i| i.is_valid()).count())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        Ok(input.iter().filter(|i| i.is_valid_2()).count())
    }
}

fn main() {
    solve::<Day2>("input").unwrap();
}