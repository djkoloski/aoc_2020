use std::{io, num};
use problem::{ProblemInput, Problem, solve};

struct Input {
    card: u32,
    door: u32,
}

#[derive(Debug)]
enum ParseInputError {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    MissingLine,
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

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();
        let card = lines.next().ok_or(ParseInputError::MissingLine)??.parse()?;
        let door = lines.next().ok_or(ParseInputError::MissingLine)??.parse()?;
        Ok(Self {
            card,
            door,
        })
    }
}

fn transform(input: u32, subject: u32, mod_size: u32) -> u32 {
    ((input as u64 * subject as u64) % mod_size as u64) as u32
}

struct Day25;
impl Problem for Day25 {
    type Input = Input;
    type Part1Output = u32;
    type Part2Output = String;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        const SUBJECT: u32 = 7;
        const MOD_SIZE: u32 = 20201227;

        let mut value = 1;
        let mut result = 1;
        while value != input.card {
            value = transform(value, SUBJECT, MOD_SIZE);
            result = transform(result, input.door, MOD_SIZE);
        }

        Ok(result)
    }

    fn part_2(_input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        Ok("All done!".to_string())
    }
}

fn main() {
    solve::<Day25>("input").unwrap();
}
