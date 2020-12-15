use std::str::FromStr;
use problem::{Problem, solve};

struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum ParsePositionError {
    InvalidLength(usize),
    InvalidCharacter(char),
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            Err(ParsePositionError::InvalidLength(s.len()))
        } else {
            Ok(Self {
                x: s[7..10].chars().try_fold(0, |acc, c| {
                    Ok(acc << 1 | match c {
                        'L' => 0,
                        'R' => 1,
                        c => return Err(ParsePositionError::InvalidCharacter(c)),
                    })
                })?,
                y: s[0..7].chars().try_fold(0, |acc, c| {
                    Ok(acc << 1 | match c {
                        'F' => 0,
                        'B' => 1,
                        c => return Err(ParsePositionError::InvalidCharacter(c)),
                    })
                })?,
            })
        }
    }
}

struct Day5;
impl Problem for Day5 {
    type Input = Vec<Position>;
    type Part1Output = u32;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        input.iter().map(|i| i.x + i.y * 8).max().ok_or(())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut found = vec![false; 128 * 8];
        for i in input {
            let index = i.x + i.y * 8;
            found[index as usize] = true;
        }
        for i in 1..128 * 8 - 1 {
            if !found[i] && found[i - 1] && found[i + 1] {
                return Ok(i);
            }
        }
        Err(())
    }
}

fn main() {
    solve::<Day5>("input").unwrap();
}
