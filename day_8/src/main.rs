use std::str::FromStr;
use std::num::ParseIntError;
use problem::{Problem, solve};

#[derive(Clone, Copy)]
enum Instruction {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidFormat(String),
    InvalidInstruction(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(e: ParseIntError) -> Self {
        ParseInstructionError::ParseIntError(e)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        let instruction = pieces.next().ok_or(ParseInstructionError::InvalidFormat(s.to_string()))?;
        let argument = pieces.next().ok_or(ParseInstructionError::InvalidFormat(s.to_string()))?.parse()?;
        Ok(match instruction {
            "acc" => Instruction::Acc(argument),
            "jmp" => Instruction::Jmp(argument),
            "nop" => Instruction::Nop(argument),
            i => return Err(ParseInstructionError::InvalidInstruction(i.to_string())),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Reachable {
    Start,
    End,
    Neither,
}

struct Day8;
impl Problem for Day8 {
    type Input = Vec<Instruction>;
    type Part1Output = i32;
    type Part2Output = i32;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut pc = 0;
        let mut acc = 0;
        let mut visited = vec![false; input.len()];
        loop {
            if visited[pc] {
                break Ok(acc);
            }
            visited[pc] = true;

            match input[pc] {
                Instruction::Acc(amount) => {
                    acc += amount;
                    pc += 1;
                },
                Instruction::Jmp(amount) => {
                    pc = (pc as isize + amount as isize) as usize;
                },
                Instruction::Nop(_) => {
                    pc += 1;
                },
            }
        }
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut reachability = vec![Reachable::Neither; input.len()];

        let mut pc = 0;
        while reachability[pc] == Reachable::Neither {
            reachability[pc] = Reachable::Start;
            pc = match input[pc] {
                Instruction::Acc(_) | Instruction::Nop(_) => pc + 1,
                Instruction::Jmp(amount) => (pc as isize + amount as isize) as usize,
            };
        }

        let mut changed = true;
        while changed {
            changed = false;

            for pc in 0..input.len() {
                if reachability[pc] == Reachable::Neither {
                    let next_pc = match input[pc] {
                        Instruction::Acc(_) | Instruction::Nop(_) => pc + 1,
                        Instruction::Jmp(amount) => (pc as isize + amount as isize) as usize,
                    };
                    if (next_pc == input.len() || reachability[next_pc] == Reachable::End) && reachability[pc] == Reachable::Neither {
                        reachability[pc] = Reachable::End;
                        changed = true;
                    }
                }
            }
        }

        let mut mod_target = None;
        for pc in 0..input.len() {
            if reachability[pc] == Reachable::Start {
                let modified_next_pc = match input[pc] { 
                    Instruction::Acc(_) | Instruction::Jmp(_) => pc + 1,
                    Instruction::Nop(amount) => (pc as isize + amount as isize) as usize,
                };
                if modified_next_pc == input.len() || reachability[modified_next_pc] == Reachable::End {
                    mod_target = Some(pc);
                    break;
                }
            }
        }

        let mod_target = mod_target.unwrap();

        let mut pc = 0;
        let mut acc = 0;
        while pc < input.len() {
            let mut instruction = input[pc];
            if pc == mod_target {
                instruction = match instruction {
                    Instruction::Jmp(amount) => Instruction::Nop(amount),
                    Instruction::Nop(amount) => Instruction::Jmp(amount),
                    i => i,
                };
            }
            match instruction {
                Instruction::Acc(amount) => {
                    acc += amount;
                    pc += 1;
                },
                Instruction::Jmp(amount) => {
                    pc = (pc as isize + amount as isize) as usize;
                },
                Instruction::Nop(_) => {
                    pc += 1;
                },
            }
        }

        Ok(acc)
    }
}

fn main() {
    solve::<Day8>("input").unwrap();
}
