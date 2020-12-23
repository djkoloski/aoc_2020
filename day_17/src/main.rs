use std::io;
use grid::Grid;
use problem::{ProblemInput, Problem, solve};

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Inactive,
    Active,
}

impl Default for State {
    fn default() -> Self {
        Self::Inactive
    }
}

struct InitialState {
    grid: Grid<State>,
}

#[derive(Debug)]
enum ParseStateError {
    IoError(io::Error),
    InvalidChar(char),
}

impl From<io::Error> for ParseStateError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl ProblemInput for InitialState {
    type Error = ParseStateError;

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut grid = Grid::new(8, 8);

        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line?.chars().enumerate() {
                let state = match c {
                    '.' => State::Inactive,
                    '#' => State::Active,
                    c => return Err(ParseStateError::InvalidChar(c)),
                };
                *grid.get_mut(x as i32, y as i32) = state;
            }
        }

        Ok(InitialState { grid })
    }
}

fn simulate_iters(grid: &Grid<State>, steps: usize) -> usize {
    let padding = steps as i32 + 1;
    let width = grid.width() as i32 + padding * 2;
    let height = grid.height() as i32 + padding * 2;
    let depth = 1 + padding * 2;
    let mut space = vec![State::Inactive; (width * height * depth) as usize];

    for (x, y) in grid.enumerate() {
        let index = x + steps as i32 + width * (y + steps as i32 + height * padding);
        space[index as usize] = *grid.get(x, y);
    }

    for _ in 0..steps {
        let mut next_space = vec![State::Inactive; (width * height * depth) as usize];

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                for z in 1..depth - 1 {
                    let mut neighbors = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            for dz in -1..=1 {
                                if dx != 0 || dy != 0 || dz != 0 {
                                    let index = x + dx + width * (y + dy + height * (z + dz));
                                    match space[index as usize] {
                                        State::Inactive => (),
                                        State::Active => neighbors += 1,
                                    }
                                }
                            }
                        }
                    }
                    let index = x + width * (y + height * z);
                    match space[index as usize] {
                        State::Inactive => {
                            if neighbors == 3 {
                                next_space[index as usize] = State::Active;
                            }
                        },
                        State::Active => {
                            if neighbors == 2 || neighbors == 3 {
                                next_space[index as usize] = State::Active;
                            }
                        },
                    }
                }
            }
        }

        space = next_space;
    }

    space.iter().filter(|&s| *s == State::Active).count()
}

fn simulate_iters_4d(grid: &Grid<State>, steps: usize) -> usize {
    let padding = steps as i32 + 1;
    let width = grid.width() as i32 + padding * 2;
    let height = grid.height() as i32 + padding * 2;
    let depth = 1 + padding * 2;
    let hyper = 1 + padding * 2;
    let mut space = vec![State::Inactive; (width * height * depth * hyper) as usize];

    for (x, y) in grid.enumerate() {
        let index = x + steps as i32 + width * (y + steps as i32 + height * (padding + depth * padding));
        space[index as usize] = *grid.get(x, y);
    }

    for _ in 0..steps {
        let mut next_space = vec![State::Inactive; (width * height * depth * hyper) as usize];

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                for z in 1..depth - 1 {
                    for w in 1..hyper - 1 {
                        let mut neighbors = 0;
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                for dz in -1..=1 {
                                    for dw in -1..=1 {
                                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                                            let index = x + dx + width * (y + dy + height * (z + dz + depth * (w + dw)));
                                            match space[index as usize] {
                                                State::Inactive => (),
                                                State::Active => neighbors += 1,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let index = x + width * (y + height * (z + depth * w));
                        match space[index as usize] {
                            State::Inactive => {
                                if neighbors == 3 {
                                    next_space[index as usize] = State::Active;
                                }
                            },
                            State::Active => {
                                if neighbors == 2 || neighbors == 3 {
                                    next_space[index as usize] = State::Active;
                                }
                            },
                        }
                    }
                }
            }
        }

        space = next_space;
    }

    space.iter().filter(|&s| *s == State::Active).count()
}

struct Day17;
impl Problem for Day17 {
    type Input = InitialState;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        Ok(simulate_iters(&input.grid, 6))
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        Ok(simulate_iters_4d(&input.grid, 6))
    }
}

fn main() {
    solve::<Day17>("input").unwrap();
}
