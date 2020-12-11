use core::{
    num::NonZeroI32,
    str::FromStr,
};
use grid::Grid;
use problem::{Problem, solve};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor
    }
}

#[derive(Debug)]
enum ParseTileError {
    InvalidChar(char),
}

impl Tile {
    fn from_char(c: char) -> Result<Self, ParseTileError> {
        Ok(match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            c => return Err(ParseTileError::InvalidChar(c)),
        })
    }
}

struct GridRow {
    pub tiles: Vec<Tile>,
}

#[derive(Debug)]
enum ParseGridRowError {
    InvalidTile {
        column: usize,
        inner: ParseTileError,
    }
}

impl FromStr for GridRow {
    type Err = ParseGridRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s.chars().enumerate().map(|(i, c)| Tile::from_char(c).map_err(|e| ParseGridRowError::InvalidTile { column: i, inner: e })).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn step_grid_neighbors(grid: &mut Grid<Tile>) -> bool {
    let neighbors = Grid::new_with(
        grid.width(),
        grid.height(),
        |x, y| {
            match grid.get(x, y) {
                Tile::Floor => 0,
                Tile::Empty | Tile::Occupied => grid.neighbors(x, y).filter(|&n| n == &Tile::Occupied).count()
            }
        }
    );

    step_grid(grid, &neighbors, 4)
}

fn step_grid_line_of_sight(grid: &mut Grid<Tile>, neighbors: &[Grid<Option<NonZeroI32>>]) -> bool {
    let neighbors = Grid::new_with(
        grid.width(),
        grid.height(),
        |x, y| neighbors.iter().enumerate().filter(|&(i, n)| {
            if let Some(dist) = n.get(x, y) {
                grid.get(x + dist.get() * OFFSET_X[i], y + dist.get() * OFFSET_Y[i]) == &Tile::Occupied
            } else {
                false
            }
        }).count()
    );

    step_grid(grid, &neighbors, 5)
}

fn step_grid(grid: &mut Grid<Tile>, neighbors: &Grid<usize>, threshold: usize) -> bool {
    let mut changed = false;
    for (x, y) in grid.enumerate() {
        match grid.get(x, y) {
            Tile::Floor => (),
            Tile::Empty => {
                if *neighbors.get(x, y) == 0 {
                    grid.set(x, y, Tile::Occupied);
                     changed = true;
                 }
            },
            Tile::Occupied => {
                if *neighbors.get(x, y) >= threshold {
                    grid.set(x, y, Tile::Empty);
                    changed = true;
                 }
            },
        }
    }

    changed
}

const OFFSET_X: [i32; 8] = [
    -1,  0,  1,
    -1,      1,
    -1,  0,  1,
];
const OFFSET_Y: [i32; 8] = [
    -1, -1, -1,
     0,      0,
     1,  1,  1,
];

fn line_of_sight(grid: &Grid<Tile>, index: usize) -> Grid<Option<NonZeroI32>> {
    Grid::new_with(
        grid.width(),
        grid.height(),
        |x, y| {
            let mut dist = 1;
            loop {
                if let Some(tile) = grid.try_get(x + OFFSET_X[index] * dist, y + OFFSET_Y[index] * dist) {
                    match tile {
                        Tile::Floor => dist += 1,
                        Tile::Empty | Tile::Occupied => break Some(NonZeroI32::new(dist).unwrap()),
                    }
                } else {
                    break None;
                }
            }
        }
    )
}

struct Day11;
impl Problem for Day11 {
    type Input = GridRow;
    type Part1Output = usize;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Vec<Self::Input>) -> Result<Self::Part1Output, Self::Error> {
        let mut grid = Grid::new_with(input[0].tiles.len(), input.len(), |x, y| input[y as usize].tiles[x as usize]);

        while step_grid_neighbors(&mut grid) {}

        Ok(grid.enumerate().map(|(x, y)| grid.get(x, y)).filter(|&t| *t == Tile::Occupied).count())
    }

    fn part_2(input: &Vec<Self::Input>) -> Result<Self::Part2Output, Self::Error> {
        let mut grid = Grid::new_with(input[0].tiles.len(), input.len(), |x, y| input[y as usize].tiles[x as usize]);
        let line_of_sight_grids = (0..8).map(|i| line_of_sight(&grid, i)).collect::<Vec<_>>();

        while step_grid_line_of_sight(&mut grid, line_of_sight_grids.as_slice()) {}

        Ok(grid.enumerate().map(|(x, y)| grid.get(x,y)).filter(|&t| *t == Tile::Occupied).count())
    }
}

fn main() {
    solve::<Day11>("input").unwrap();
}
