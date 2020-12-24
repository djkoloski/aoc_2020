use std::{collections::HashMap, str::FromStr};

use problem::{Problem, solve};

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn step(&self, pos: (i32, i32)) -> (i32, i32) {
        let (x, y) = pos;
        match self {
            Direction::East => (x + 1, y),
            Direction::NorthEast => (x, y + 1),
            Direction::NorthWest => (x - 1, y + 1),
            Direction::West => (x - 1, y),
            Direction::SouthWest => (x, y - 1),
            Direction::SouthEast => (x + 1, y - 1),
        }
    }
}

struct Trail {
    directions: Vec<Direction>,
}

#[derive(Debug)]
enum ParseTrailError {
    InvalidDirection(char),
    UnexpectedEndOfInput,
}

impl FromStr for Trail {
    type Err = ParseTrailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions = Vec::new();

        let mut chars = s.chars();
        while let Some(d) = chars.next() {
            directions.push(match d {
                'e' => Direction::East,
                'w' => Direction::West,
                's' => match chars.next() {
                    Some('e') => Direction::SouthEast,
                    Some('w') => Direction::SouthWest,
                    Some(e) => return Err(ParseTrailError::InvalidDirection(e)),
                    None => return Err(ParseTrailError::UnexpectedEndOfInput),
                },
                'n' => match chars.next() {
                    Some('e') => Direction::NorthEast,
                    Some('w') => Direction::NorthWest,
                    Some(e) => return Err(ParseTrailError::InvalidDirection(e)),
                    None => return Err(ParseTrailError::UnexpectedEndOfInput),
                },
                e => return Err(ParseTrailError::InvalidDirection(e)),
            });
        }

        Ok(Self {
            directions,
        })
    }
}

struct Day24;
impl Problem for Day24 {
    type Input = Vec<Trail>;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let mut tiles = HashMap::new();

        for trail in input.iter() {
            let position = trail.directions.iter().fold((0, 0), |pos, direction| direction.step(pos));
            let tile = tiles.entry(position).or_insert(false);
            *tile = !*tile;
        }

        Ok(tiles.values().filter(|&x| *x).count())
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let mut tiles = HashMap::new();

        for trail in input.iter() {
            let position = trail.directions.iter().fold((0, 0), |pos, direction| direction.step(pos));
            let tile = tiles.entry(position).or_insert(false);
            *tile = !*tile;
        }

        for _ in 0..100 {
            let mut neighbors = HashMap::new();
            for (pos, _) in tiles.iter().filter(|&(_, tile)| *tile) {
                neighbors.entry(*pos).or_insert(0);
                for direction in &[Direction::East, Direction::NorthEast, Direction::NorthWest, Direction::West, Direction::SouthWest, Direction::SouthEast] {
                    let neighbor = direction.step(*pos);
                    *neighbors.entry(neighbor).or_insert(0) += 1;
                }
            }
            for (pos, count) in neighbors.iter() {
                let tile = tiles.entry(*pos).or_insert(false);
                if match *tile {
                    true => *count == 0 || *count > 2,
                    false => *count == 2,
                } {
                    *tile = !*tile;
                }
            }
        }

        Ok(tiles.values().filter(|&x| *x).count())
    }
}

fn main() {
    solve::<Day24>("input").unwrap();
}