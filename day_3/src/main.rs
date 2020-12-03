use std::str::FromStr;

use problem::{Problem, solve};

enum Spot {
    Empty,
    Tree,
}

struct TreeLine {
    spots: Vec<Spot>,
}

#[derive(Debug)]
enum ParseTreeLineError {
    InvalidChar(char),
}

impl FromStr for TreeLine {
    type Err = ParseTreeLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spots = Vec::new();
        for c in s.chars() {
            let spot = match c {
                '.' => Spot::Empty,
                '#' => Spot::Tree,
                c => return Err(ParseTreeLineError::InvalidChar(c)),
            };
            spots.push(spot);
        }
        Ok(Self { spots })
    }
}

#[derive(Debug)]
enum Error {}

fn hit_trees(input: &Vec<TreeLine>, slope_x: usize, slope_y: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    while y < input.len() {
        let line = &input[y];
        match line.spots[x % line.spots.len()] {
            Spot::Tree => trees += 1,
            _ => (),
        }
        x += slope_x;
        y += slope_y;
    }
    trees
}

struct Day3;
impl Problem for Day3 {
    type Input = TreeLine;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = Error;

    fn part_1(input: &Vec<Self::Input>) -> Result<Self::Part1Output, Self::Error> {
        Ok(hit_trees(input, 3, 1))
    }

    fn part_2(input: &Vec<Self::Input>) -> Result<Self::Part2Output, Self::Error> {
        Ok(
            hit_trees(input, 1, 1)
            * hit_trees(input, 3, 1)
            * hit_trees(input, 5, 1)
            * hit_trees(input, 7, 1)
            * hit_trees(input, 1, 2)
        )
    }
}

fn main() {
    solve::<Day3>("input").unwrap();
}
