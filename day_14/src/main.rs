use std::{collections::HashMap, num::ParseIntError, str::FromStr};
use problem::{Problem, solve};

enum Instruction {
    SetMask {
        value: u64,
        mask: u64,
    },
    SetMem {
        address: u64,
        value: u64,
    },
}

#[derive(Debug)]
enum ParseInstructionError {
    ParseIntError(ParseIntError),
    InvalidInstruction,
    MissingEquals,
    InvalidBit(char),
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let mut pieces = s.split('=');
            pieces.next();
            let (value, mask) = pieces.next()
                .ok_or(ParseInstructionError::MissingEquals)?
                .trim_start()
                .chars()
                .rev()
                .enumerate()
                .try_fold((0, 0), |(value, mask), (i, bit)| {
                    let one = 1 << i;
                    Ok(match bit {
                        '0' => (value, mask),
                        '1' => (value | one, mask),
                        'X' => (value, mask | one),
                        c => return Err(ParseInstructionError::InvalidBit(c)),
                    })
                })?;
            Ok(Instruction::SetMask {
                value,
                mask,
            })
        } else if s.starts_with("mem") {
            let mut pieces = s.split('=');
            let address = pieces.next().unwrap().trim_end();
            let address = address[4..address.len() - 1].parse()?;
            let value = pieces.next().ok_or(ParseInstructionError::MissingEquals)?.trim_start().parse()?;
            Ok(Instruction::SetMem {
                address,
                value,
            })
        } else {
            Err(ParseInstructionError::InvalidInstruction)
        }
    }
}

struct Day14;
impl Problem for Day14 {
    type Input = Vec<Instruction>;
    type Part1Output = u64;
    type Part2Output = u64;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut values = HashMap::new();
        let mut mask_value = 0;
        let mut mask_mask = 0;
        for i in input.iter() {
            match i {
                Instruction::SetMask { value, mask } => {
                    mask_value = *value;
                    mask_mask = *mask;
                },
                Instruction::SetMem { address, value } => {
                    let write_value = mask_value | (value & mask_mask);
                    values.insert(*address, write_value);
                },
            }
        }

        Ok(values.values().sum())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut values = HashMap::new();

        let mut mask_value = 0;
        let mut mask_mask = 0;
        for instruction in input.iter() {
            match instruction {
                Instruction::SetMask { value, mask } => {
                    mask_value = *value;
                    mask_mask = *mask;
                },
                Instruction::SetMem { address, value } => {
                    fn set_value(address: u64, mask: u64, value: u64, values: &mut HashMap<u64, u64>) {
                        let zeros = mask.trailing_zeros();
                        if zeros == 64 {
                            values.insert(address, value);
                        } else {
                            let bit = 1 << zeros;
                            let next_mask = mask & !bit;
                            set_value(address, next_mask, value, values);
                            set_value(address | bit, next_mask, value, values);
                        }
                    }
                    set_value((address | mask_value) & !mask_mask, mask_mask, *value, &mut values);
                },
            }
        }

        Ok(values.values().sum())
    }
}

fn main() {
    solve::<Day14>("input").unwrap();
}
