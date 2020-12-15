use core::{
    num::ParseIntError,
    str::FromStr,
};
use problem::{Problem, solve};

enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidInstruction(String),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" => Instruction::North,
            "S" => Instruction::South,
            "E" => Instruction::East,
            "W" => Instruction::West,
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            "F" => Instruction::Forward,
            d => return Err(ParseInstructionError::InvalidInstruction(d.to_string())),
        })
    }
}

struct Action {
    instruction: Instruction,
    argument: i32,
}

#[derive(Debug)]
enum ParseActionError {
    InvalidInstruction(ParseInstructionError),
    InvalidArgument(ParseIntError),
}

impl From<ParseInstructionError> for ParseActionError {
    fn from(e: ParseInstructionError) -> Self {
        Self::InvalidInstruction(e)
    }
}

impl From<ParseIntError> for ParseActionError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidArgument(e)
    }
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = s[0..1].parse()?;
        let argument = s[1..].parse()?;
        Ok(Self {
            instruction,
            argument,
        })
    }
}

struct Day12;
impl Problem for Day12 {
    type Input = Vec<Action>;
    type Part1Output = i32;
    type Part2Output = i32;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        const OFFSET_X: [i32; 4] = [1, 0, -1, 0];
        const OFFSET_Y: [i32; 4] = [0, 1, 0, -1];

        let mut x = 0;
        let mut y = 0;
        let mut r = 0;

        for action in input {
            match action.instruction {
                Instruction::North => y += action.argument,
                Instruction::South => y -= action.argument,
                Instruction::East => x += action.argument,
                Instruction::West => x -= action.argument,
                Instruction::Right => r = (r + 4 - action.argument as usize / 90 % 4) % 4,
                Instruction::Left => r = (r + action.argument as usize / 90) % 4,
                Instruction::Forward => {
                    x += OFFSET_X[r] * action.argument;
                    y += OFFSET_Y[r] * action.argument;
                },
            }
        }

        Ok(i32::abs(x) + i32::abs(y))
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut x = 0;
        let mut y = 0;
        let mut wx = 10;
        let mut wy = 1;

        for action in input {
            let rx = [wx, -wy, -wx, wy];
            let ry = [wy, wx, -wy, -wx];

            match action.instruction {
                Instruction::North => wy += action.argument,
                Instruction::South => wy -= action.argument,
                Instruction::East => wx += action.argument,
                Instruction::West => wx -= action.argument,
                Instruction::Right => {
                    let r = (4 - action.argument as usize / 90 % 4) % 4;
                    wx = rx[r];
                    wy = ry[r];
                },
                Instruction::Left => {
                    let r = action.argument as usize / 90 % 4;
                    wx = rx[r];
                    wy = ry[r];
                },
                Instruction::Forward => {
                    x += action.argument * wx;
                    y += action.argument * wy;
                },
            }
        }

        Ok(i32::abs(x) + i32::abs(y))
    }
}

fn main() {
    solve::<Day12>("input").unwrap();
}
